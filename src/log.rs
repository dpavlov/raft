pub enum OpType {
    INSERT,
    UPDATE,
    DELETE
}

pub struct Entry {
    pub key: String,
    pub value: Option<String>,
}

pub struct Operation {
    pub term: i64,
    pub optype: OpType,
    pub entry: Entry,
}

impl Operation {
    pub fn insert(term: i64, key: String, value: String) -> Self {
        Operation {
            term,
            optype: OpType::INSERT,
            entry: Entry {
                key,
                value: Some(value)
            }
        }
    }

    pub fn update(term: i64, key: String, value: String) -> Self {
        Operation {
            term,
            optype: OpType::UPDATE,
            entry: Entry {
                key,
                value: Some(value)
            }
        }
    }

    pub fn delete(term: i64, key: String) -> Self {
        Operation {
            term,
            optype: OpType::UPDATE,
            entry: Entry {
                key,
                value: Option::None
            }
        }
    }
}

pub struct  OperationLog {
    operations: Vec<Operation>
}

impl OperationLog {

    pub fn new() -> Self {
        OperationLog {
            operations: vec![]
        }
    }

    pub fn append(&mut self, operation: Operation) {
        self.operations.push(operation);
    }

    pub fn get(&self, index: usize) -> &Operation {
        &self.operations[index]
    }

    pub fn all(&self) -> &[Operation] {
        &self.operations
    }

    pub fn last_index(&self) -> usize {
        self.operations.len() - 1
    }

    pub fn last_term(&self) -> i64 {
        self.term_for(self.last_index())
    }

    pub fn term_for(&self, index: usize) -> i64 {
        if self.operations.is_empty() {
            0_i64
        } else {
            self.operations[index].term
        }
    }

    pub fn truncate(&mut self, start_index: usize) {
        let end_index = self.operations.len();
        self.operations.drain(start_index .. end_index).collect::<Vec<Operation>>();
    }
}