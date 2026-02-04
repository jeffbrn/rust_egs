trait IntCmd {
    #[allow(dead_code)]
    fn execute(&self, value: i32) -> Result<i32, &'static str>;

    #[allow(dead_code)]
    fn undo(&self, value: i32) -> Result<i32, &'static str>;
}

#[allow(dead_code)]
fn exec(cmds: &Vec<Box<dyn IntCmd>>, initial: i32) -> Result<i32, &'static str> {
    let result = cmds.iter().try_fold(initial, |acc, cmd| cmd.execute(acc))?;
    Ok(result)
}

#[allow(dead_code)]
fn revert(cmds: &Vec<Box<dyn IntCmd>>, result: i32) -> Result<i32, &'static str> {
    let result = cmds.iter().try_fold(result, |acc, cmd| cmd.undo(acc))?;
    Ok(result)
}

#[allow(dead_code)]
struct AddCmd {
    operand: i32,
}

impl AddCmd {
    #[allow(dead_code)]
    pub fn new(operand: i32) -> Self {
        Self { operand }
    }
}

impl IntCmd for AddCmd {
    fn execute(&self, value: i32) -> Result<i32, &'static str> {
        Ok(self.operand + value)
    }

    fn undo(&self, value: i32) -> Result<i32, &'static str> {
        Ok(value - self.operand)
    }
}

#[allow(dead_code)]
struct MultCmd {
    operand: i32,
}

impl MultCmd {
    #[allow(dead_code)]
    pub fn new(operand: i32) -> Self {
        Self { operand }
    }
}

impl IntCmd for MultCmd {
    fn execute(&self, value: i32) -> Result<i32, &'static str> {
        Ok(self.operand * value)
    }

    fn undo(&self, value: i32) -> Result<i32, &'static str> {
        let retval = if value == 0 {
            Err("Cannot undo multiplication by zero")
        } else {
            let result: i32 = value / self.operand;
            Ok(result)
        };
        retval
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_int_cmds() {
        let initial = 0;
        let cmds: Vec<Box<dyn IntCmd>> = vec![
            Box::new(AddCmd::new(5)),
            Box::new(AddCmd::new(10)),
            Box::new(AddCmd::new(15)),
        ];

        let result = exec(&cmds, initial).unwrap();
        assert_eq!(result, 30);
        let unwind = revert(&cmds, result).unwrap();
        assert_eq!(unwind, initial);
    }

    #[test]
    fn test_bad_undo() {
        let initial = 2;
        let cmds: Vec<Box<dyn IntCmd>> = vec![
            Box::new(MultCmd::new(5)),
            Box::new(MultCmd::new(0)), // This will cause an undo error
            Box::new(MultCmd::new(3)),
        ];

        let result = exec(&cmds, initial).unwrap();
        assert_eq!(result, 0); // 2 * 5 * 0 * 3 = 0

        let unwind = revert(&cmds, result);
        assert!(unwind.is_err());
    }
}
