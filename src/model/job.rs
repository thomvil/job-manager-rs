use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Job {
    pub(crate) id:           Option<usize>,
    pub(crate) load:         String,
    pub(crate) difficulty:   u8,
    pub(crate) started_at:   Option<DateTime<Local>>,
    pub(crate) completed_at: Option<DateTime<Local>>,
    pub(crate) persistant:   bool,
    pub(crate) started_by:   Option<String>,
    pub(crate) result:       Option<String>,
}

// Constructors
impl Job {
    pub fn new<S: Into<String>>(load: S, difficulty: u8, persistant: bool) -> Self {
        Self {
            load: load.into(),
            difficulty,
            persistant,
            started_at: None,
            started_by: None,
            completed_at: None,
            result: None,
            id: None,
        }
    }

    pub fn start(mut self, by: &str) -> Self {
        self.started_at = Some(Local::now());
        self.started_by = Some(by.into());
        self
    }

    pub fn finish<S: Into<String>>(mut self, result: S) -> Self {
        self.result = Some(result.into());
        self.completed_at = Some(Local::now());
        self
    }
}

// Inspectors
impl Job {
    pub fn is_started(&self) -> bool {
        self.started_by.is_some()
    }

    pub fn is_completed(&self) -> bool {
        self.is_started() && self.completed_at.is_some()
    }

    pub fn can_start(&self) -> bool {
        !self.is_started() || !self.persistant
    }

    pub fn is_db_stored(&self) -> bool {
        self.id.is_some()
    }
}

// To SQL strings
impl Job {
    pub fn to_sql_insert(&self) -> String {
        format!("('{}',{},{})", self.load, self.difficulty, self.persistant)
    }

    // pub fn to_sql_update(&self) -> String {
    //     format!(
    //         "('{}',{},{},{},{},{},{})",
    //         self.load,
    //         self.difficulty,
    //         self.persistant,
    //         self.started_at.unwrap_or(""),
    //         self.started_by.unwrap_or(""),
    //         self.completed_at.unwrap_or(""),
    //         self.result.unwrap_or("")
    //     )
    // }
}

#[cfg(test)]
mod tests {
    use super::Job;

    #[test]
    fn construct() {
        let j1 = Job::new("{ load: 'test' }", 1, false);
        assert!(j1.id.is_none());
        assert!(j1.started_at.is_none());
        assert!(j1.started_by.is_none());
        assert!(j1.completed_at.is_none());
        assert!(j1.result.is_none());
    }

    #[test]
    fn start() {
        let j1 = Job::new("{ load: 'test' }", 1, false);
        let j1_started = j1.start("foobar");
        assert_eq!("foobar", j1_started.started_by.as_ref().unwrap());
        assert!(j1_started.started_at.is_some());
        assert!(j1_started.is_started());
    }

    #[test]
    fn finish() {
        let j1 = Job::new("{ load: 'test' }", 1, false).start("foobar");
        let j1_finished = j1.finish("{ load: 'test', result: 'baz'");
        assert!(j1_finished.is_completed());
    }

    #[test]
    fn can_start() {
        let j1 = Job::new("{ load: 'test' }", 1, false)
            .start("foo")
            .finish("bar");
        assert!(j1.is_started());
        assert!(!j1.persistant);
        assert!(j1.can_start());

        let j2 = Job::new("{ load: 'test' }", 1, true);
        assert!(!j2.is_started());
        assert!(j2.can_start());

        let j3 = Job::new("{ load: 'test' }", 1, true).start("foo");
        assert!(j3.is_started());
        assert!(j3.persistant);
        assert!(!j3.can_start());
    }
}
