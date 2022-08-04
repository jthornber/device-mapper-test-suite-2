use anyhow::Result;
use devicemapper::{DevId, DmFlags, DmName, DmOptions, DM};

use crate::device_mapper::interface::*;

//----------------------------------

pub struct Ioctl {
    control: DM,
}

impl Ioctl {
    pub fn new() -> Result<Self> {
        Ok(Self {
            control: DM::new()?,
        })
    }
}

/// Converts from an interface::Table to Vec<(offset, len, type, args)>
fn mk_ioctl_table(table: &Table) -> Vec<(u64, u64, String, String)> {
    let mut ts = Vec::new();
    let mut offset = 0u64;

    for t in &table.targets {
        let len = t.sectors();

        ts.push((offset, len, t.target_type().to_string(), t.args()));
        offset += len;
    }
    ts
}

impl Interface for Ioctl {
    fn create(&mut self, id: &DmName) -> Result<()> {
        let _info = self
            .control
            .device_create(&id, None, DmOptions::default())?;
        Ok(())
    }

    fn load(&mut self, id: &DmName, table: &Table) -> Result<()> {
        let id = DevId::Name(id);

        self.control
            .table_load(&id, &mk_ioctl_table(table), DmOptions::default())?;
        Ok(())
    }

    fn suspend(&mut self, id: &DmName, flush: bool) -> Result<()> {
        let id = DevId::Name(id);
        let mut flags = DmFlags::DM_SUSPEND;
        if !flush {
            flags |= DmFlags::DM_NOFLUSH
        };
        let opts = DmOptions::default().set_flags(flags);
        self.control.device_suspend(&id, opts)?;
        Ok(())
    }

    fn resume(&mut self, id: &DmName) -> Result<()> {
        let id = DevId::Name(id);
        self.control.device_suspend(&id, DmOptions::default())?;
        Ok(())
    }

    fn remove(&mut self, id: &DmName) -> Result<()> {
        let id = DevId::Name(id);
        self.control.device_remove(&id, DmOptions::default())?;
        Ok(())
    }

    fn message(&mut self, id: &DmName, sector: Option<u64>, msg: &str) -> Result<()> {
        let id = DevId::Name(id);
        self.control.target_msg(&id, sector, msg)?;
        Ok(())
    }

    fn status(&mut self, id: &DmName) -> Result<String> {
        let id = DevId::Name(id);
        let table = self.control.table_status(&id, DmOptions::default())?;
        Ok(table
            .1
            .into_iter()
            .map(|(_offset, _len, _ttype, status)| status)
            .collect())
    }

    fn table(&mut self, id: &DmName) -> Result<Table> {
        let id = DevId::Name(id);
        let table = self.control.table_status(
            &id,
            DmOptions::default().set_flags(DmFlags::DM_STATUS_TABLE),
        )?;
        let table = Table {
            targets: table
                .1
                .into_iter()
                .map(|(_offset, sectors, ttype, args)| from_target_line(sectors, &ttype, &args))
                .collect::<Result<Vec<Box<dyn Target>>>>()?,
        };
        Ok(table)
    }

    fn wait(&mut self, id: &DmName, event_nr: u64) -> Result<(u64, Vec<String>)> {
        /*
        let id = to_id(dev)?;
        // FIXME: I'm not sure setting the event_nr is right, there's a 16 bit shift in the ioctl code for some reason
        let (info, status) = self.control.device_wait(
            &id,
            DmOptions::default().set_cookie(DmCookie::from_bits_unchecked(event_nr)),
        )?;
        Ok(
            info.event_nr,
            status
                .into_iter
                .map(|(_offset, _len, _ttype, status)| stats)
                .collect(),
        )
        */
        todo!();
    }
}

//----------------------------------
