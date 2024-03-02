mod min_stack {
    struct MinStack {
        inner: Vec<(i32, i32)>,
    }

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl MinStack {
        fn new() -> Self {
            Self { inner: vec![] }
        }

        fn push(&mut self, val: i32) {
            let next_min = val.min(self.inner.last().unwrap_or(&(0, val)).1);
            self.inner.push((val, next_min));
        }

        fn pop(&mut self) {
            let _ = self.inner.pop();
        }

        fn top(&mut self) -> i32 {
            self.inner.last().unwrap().0
        }

        fn get_min(&self) -> i32 {
            self.inner.last().unwrap().1
        }
    }

    /*
     * Your MinStack object will be instantiated and called as such:
     * let obj = MinStack::new();
     * obj.push(val);
     * obj.pop();
     * let ret_3: i32 = obj.top();
     * let ret_4: i32 = obj.get_min();
     */
}

mod reverse_polish {
    //! https://leetcode.com/problems/evaluate-reverse-polish-notation/

    enum Token {
        Num(i32),
        Op(Operation),
    }

    enum Operation {
        Add,
        Sub,
        Mul,
        Div,
    }

    impl From<&str> for Operation {
        fn from(value: &str) -> Self {
            use Operation as Op;
            match value.chars().next().unwrap() {
                '+' => Op::Add,
                '-' => Op::Sub,
                '*' => Op::Mul,
                '/' => Op::Div,

                _ => panic!("invalid"),
            }
        }
    }

    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for t in tokens.iter().map(parse_token) {
            match t {
                Token::Num(num) => stack.push(num),
                Token::Op(op) => {
                    let res = eval(op, stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(res);
                }
            }
        }
        *stack.last().unwrap()
    }

    fn parse_token(s: &String) -> Token {
        match s.parse::<i32>() {
            Ok(num) => Token::Num(num),
            Err(_) => Token::Op(Operation::from(s.as_str())),
        }
    }

    fn eval(op: Operation, rhs: i32, lhs: i32) -> i32 {
        match op {
            Operation::Add => lhs + rhs,
            Operation::Sub => lhs - rhs,
            Operation::Div => lhs / rhs,
            Operation::Mul => lhs * rhs,
        }
    }
}
