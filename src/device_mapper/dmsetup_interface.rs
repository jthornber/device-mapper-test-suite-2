use anyhow::Result;
use regex::*;

use crate::device_mapper::interface::*;
use crate::process::*;
use crate::utils::*;

//----------------------------------------

struct DMSetup {}

struct Output {
    stdout: String,
    stderr: String,
}

impl DMSetup {
    fn run<S: AsRef<str>>(&self, args: &[S]) -> Result<(String, String)> {
        todo!();
    }

    fn parse_event_nr(s: &str) -> Result<u64> {
        todo!();
    }
}

/*
impl Interface for DMSetup {
    fn create(&mut self, dev: &Device) -> Result<()> {
        self.run(&vec!["create", dev.path(), "--notable"])?;
        Ok(())
    }

    fn load(&mut self, dev: &Device, table: Table) -> Result<()> {
        let table_file = with_temp_file(table)?;
        self.run(&vec!["load", dev.path(), table_file.path()])?;
        Ok(())
    }

    fn suspend(&mut self, dev: &Device, flush: bool) -> Result<()> {
        if flush {
            self.run(&vec!["suspend", dev.path()])
        } else {
            self.run(&vec!["suspend", "--noflush", dev.path()])
        }
    }

    fn resume(&mut self, dev: &Device) -> Result<()> {
        self.run(&vec!["resume", dev.path()])?;
        Ok(())
    }

    fn remove(&mut self, dev: &Device) -> Result<()> {
        self.run(&vec!["remove", dev.path()])?;
        Ok(())
    }

    fn message(&mut self, dev: &Device, sector: u64, args: &[String]) -> Result<()> {
        let mut cmd = vec!["message", &format!("{}", sector)];
        cmd.append(&mut args);
        self.run(&cmd)?;
        Ok(())
    }

    fn status(&mut self, dev: &Device) -> Result<String> {
        let r = self.run(&vec!["status", dev.path()])?;
        Ok(r.stdout)
    }

    fn table(&mut self, dev: &Device) -> Result<Table> {
        let r = self.run(&vec!["table", dev.path()])?;
        Ok(r.stdout)
    }

    fn info(&mut self, dev: &Device) -> Result<String> {
        let r = self.run(&vec!["info", dev.path()])?;
        Ok(r.stdout)
    }

    fn wait(&mut self, dev: &Device, event_nr: u64) -> Result<u64> {
        let r = self.run(&vec!["wait", &format!("{}", event_nr)])?;
        Ok(Self::parse_event_nr(r.stdout)?)
    }
}
*/

//----------------------------------------
