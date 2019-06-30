struct Context<'a>(&'a str);

struct Parser<'c, 's:'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}


trait Red { }

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> { }

fn main() {
    parse_context(Context("hello"));


    let num = 5;
    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
}
