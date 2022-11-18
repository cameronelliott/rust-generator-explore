#![feature(generators, generator_trait)]

use core::ops::Generator;
use core::ops::GeneratorState;
use core::pin::Pin;
use either::Either;

#[propane::generator]
fn fizz_buzzs() -> &'static str {
    for x in 1..10009000 {
        match (x % 3 == 0, x % 5 == 0) {
            (true, true) => yield "FizzBuzz",
            (true, false) => yield "Fizz",
            (false, true) => yield "Buzz",
            (..) => yield "anum",
        }
    }
}
#[propane::generator]
fn fizz_buzz() -> i32 {
    for x in 1..10009000 {
        match (x % 3 == 0, x % 5 == 0) {
            (true, true) => yield 1,
            (true, false) => yield 2,
            (false, true) => yield 3,
            (..) => yield 4,
        }
    }
}

// #[propane::generator]
// fn g2() ->i32 {

//     for x in 1..101 {
//         let a = yield;
//         println!("g2 {}", a);
//     }
// }

fn main() {
    //    for x in fizz_buzz() {
    //     println!("x {}",x);
    //    }

    let mut mysrc = move || {
        for x in 1..1000000{
            yield x;
        }
    };

    let mut mysink = move |_: i32| loop {
        let a = yield;
        //println!("got a {}", a);
    };

    let mut a = Supply {
        input: mysrc,
        sink: mysink,
    };
    let t = std::time::Instant::now();
    let mut n = 0;
    // loop {
    //     match Pin::new(&mut mysink).resume(9) {
    //         //GeneratorState::Yielded(_) => todo!(),
    //         GeneratorState::Complete(_) => break,
    //         GeneratorState::Yielded(_) => (),
    //     }
    //     n += 1;
    // }
    let mut z=0;
    for x in fizz_buzz() {
        z+=x;
        n+=1;
    }
    let el = t.elapsed();
    println!("z {}",z);
    println!("n {}",n);
    println!("el {:?}", el);
    println!("ns/iter {:?}",el.as_nanos() as f64/(n as f64));
    // println!("x {:?}", Pin::new(&mut a).resume(()));
    // println!("x {:?}", Pin::new(&mut a).resume(()));
    // println!("x {:?}", Pin::new(&mut a).resume(()));
    // println!("x {:?}", Pin::new(&mut a).resume(()));
    // println!("x {:?}", Pin::new(&mut a).resume(()));
    // println!("x {:?}", Pin::new(&mut a).resume(()));
    // println!("x {:?}", Pin::new(&mut a).resume(()));
    // println!("x {:?}", Pin::new(&mut a).resume(()));
}

// pub trait Generator<Arg> {
//     type Yield;
//     type Return;

//     fn resume(self: Pin<&mut Self>, arg: Arg) -> GeneratorState<Self::Yield, Self::Return>;
// }

struct Supply<In, Sink> {
    input: In,
    sink: Sink,
}

impl<Arg, In, Sink> Generator<Arg> for Supply<In, Sink>
where
    In: Generator<Arg>,
    Sink: Generator<In::Yield>,
{
    type Yield = Sink::Yield;
    type Return = Either<In::Return, Sink::Return>;

    fn resume(self: Pin<&mut Self>, arg: Arg) -> GeneratorState<Self::Yield, Self::Return> {
        unsafe {
            let Supply { input, sink } = Pin::get_unchecked_mut(self);

            let input = Pin::new_unchecked(input);
            let sink = Pin::new_unchecked(sink);

            let arg = match input.resume(arg) {
                GeneratorState::Yielded(y) => y,
                GeneratorState::Complete(ret) => {
                    return GeneratorState::Complete(Either::Left(ret))
                }
            };
            //   println!("99");
            match sink.resume(arg) {
                GeneratorState::Yielded(y) => GeneratorState::Yielded(y),
                GeneratorState::Complete(ret) => GeneratorState::Complete(Either::Right(ret)),
            }
        }
    }
}
