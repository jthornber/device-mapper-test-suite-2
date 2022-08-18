use anyhow::{anyhow, Result};
use bitflags::bitflags;
use devicemapper::Device;
use log::*;
use std::collections::{HashSet, VecDeque};

//--------------------------

bitflags! {
    pub struct Tags: u32 {
        const NVME = 0b0001;
        const SPINDLE = 0b0010;
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct Segment {
    pub dev: Device,
    pub b_sector: u64,
    pub e_sector: u64,
    pub tags: Tags,
}

impl Segment {
    pub fn len(&self) -> u64 {
        self.e_sector - self.b_sector
    }

    pub fn is_empty(&self) -> bool {
        self.e_sector == self.b_sector
    }
}

#[derive(Default)]
pub struct Allocator {
    free: HashSet<Segment>,
}

impl Allocator {
    pub fn add_allocation_seg(&mut self, seg: Segment) {
        self.free.insert(seg);
    }

    pub fn total_free(&self) -> u64 {
        self.free.iter().map(|s| s.len()).fold(0, |x, y| x + y)
    }

    fn free<I: Iterator<Item = Segment>>(&mut self, segs: I) {
        for s in segs {
            self.free.insert(s);
        }
    }

    // All `tags` must be present in the segment
    pub fn alloc_tagged(&mut self, len: u64, tags: Tags) -> Result<Vec<Segment>> {
        debug!("free {:?}", self.free);
        let mut segs: VecDeque<Segment> = self
            .free
            .drain_filter(|s| (s.tags & tags) == tags)
            .collect();

        debug!("segs {:?}", segs);

        let mut result = Vec::new();
        let mut remaining = len;
        while remaining > 0 {
            match segs.pop_front() {
                None => {
                    // Undo the allocation
                    self.free(result.into_iter());
                    self.free(segs.into_iter());
                    return Err(anyhow!("segment allocator out of space"));
                }
                Some(s) => {
                    if s.len() > remaining {
                        segs.push_back(Segment {
                            dev: s.dev.clone(),
                            b_sector: s.b_sector + remaining,
                            e_sector: s.e_sector,
                            tags: s.tags,
                        });
                    }
                    remaining -= s.len();
                    result.push(s);
                }
            }
        }

        Ok(result)
    }

    pub fn alloc(&mut self, len: u64) -> Result<Vec<Segment>> {
        self.alloc_tagged(len, Tags::empty())
    }
}

//--------------------------
