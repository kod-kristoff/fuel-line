#![feature(proc_macro)]
#![feature(test)]

// For tests
#[allow(unused_imports)]
extern crate bytes;
#[allow(unused_imports)]
#[macro_use] extern crate fuel_line;
#[macro_use] extern crate fuel_line_derive;
#[allow(unused_imports)]
extern crate uuid;
extern crate test;

pub trait Render {
  fn render(&self) -> String;
}

#[cfg(test)]
mod tests {
  #![feature(proc_macro)]

  use bytes::{BytesMut, BufMut};
  use test::Bencher;
  use Render;
  use uuid::Uuid;

  #[bench]
  fn bench_render_derive(bencher: &mut Bencher) {
    #[derive(Render)]
    #[TemplateName = "./fuel_line/test_data/bench_template.txt"]
    struct BenchTemplate {
      a: String,
      b: String
    }

    let mut t = BenchTemplate {
      a: "".to_owned(),
      b: "Title".to_owned()
    };

    bencher.iter(|| {
      let a = Uuid::new_v4().to_string();
      t.a = a;

      t.render()
    });
  }

  #[bench]
  fn bench_buffer_concat(bencher: &mut Bencher) {
    let b = "Title";

    bencher.iter(|| {
      let a = Uuid::new_v4().to_string();

      let mut url = String::with_capacity(1 + ": ".len() + a.len() + b.len());
      url.push_str(b);
      url.push_str(": ");
      url.push_str(&a);
      url.push_str("\n");
      url
    });
  }
}

