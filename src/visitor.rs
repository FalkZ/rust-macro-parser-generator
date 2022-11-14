use crate::{renderer::{Function, Variable, Modifiers, Statements, Body}};

use crate::grammar::{arguments,  modifiers, modifiers_single, statements, maybe_arguments, function, variable, modifier, statement};



pub struct Visitor;


impl Visitor {
    fn arguments(&self, args: &arguments) -> Vec<String> {
        let mut vec: Vec<String> = args.arguments.iter().map(|a| a.arg.0.clone()).collect();

        vec.push(args.last.0.clone());

        vec
    }

    fn function(&self, f: &function) -> Function {
        let name = *f.name.to_owned();

        let args = match *f.arguments.clone() {
            maybe_arguments::arguments(a) => self.arguments(&a),
            maybe_arguments::no_arguments(_) => vec![],
        };

        let mut modifiers = self.modifiers(&f.modifiers);

     

        modifiers.mutable = true;

        Function {
            name,
            args,
            modifiers,
            body: Body(*f.body.to_owned())
        }
    }

    fn variable(&self, f: &variable) -> Variable {
        let name = f.name.0.to_owned();
        let modifiers = self.modifiers(&f.modifiers);

        Variable { name, modifiers }
    }

    fn modifiers(&self, m: &modifiers) -> Modifiers {
        let mut r = Modifiers::default();
        m.iter().for_each(|b: &modifiers_single| {
            match *b.modifier {
                modifier::CR => r.cr = true,
                modifier::MUT => r.mutable = true,
                modifier::PUB => r.public = true,
            };
        });

        r
    }

    fn statement(&self, s: &mut Statements, statement: &statement) {
        match statement {
            statement::function(fun) => {
                s.functions.push(self.function(&fun));
            }
            statement::variable(var) => {
                s.variables.push(self.variable(&var));
            }
        };
    }

    pub fn statements(&self, statements: &statements) -> Statements {
        let mut s = Statements::default();

        statements
            .iter()
            .for_each(|v| self.statement(&mut s, &v.statement));

        s
    }
}