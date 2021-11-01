#![feature(default_free_fn)]
#![feature(associated_type_bounds)]
#![feature(generators)]

use rand::{distributions::Alphanumeric, Rng};

use std::time::{Instant, SystemTime, UNIX_EPOCH};
use doublets::doublets::{ILinks, ILinksExtensions};
use doublets::doublets::data::LinksConstants;
use doublets::doublets::mem::united::Links;
use doublets::mem::{FileMappedMem, HeapMem};
use crate::db_context::{BlogPost, BlogPosts, DoubletsDbContext, POSTS, PropertiesOperator};
use crate::sequences::unicode::{CharToUnicode, StringToUnicode, UnicodeToChar, UnicodeToString};
use crate::test_run::DoubletsTest;

mod sequences;
pub mod db_context;
mod test_run;

const COUNT: usize = 1_000_000;

//#[test]
fn bench_doublets() {
    std::fs::remove_file("test.links");
    BlogPosts::generate(COUNT);

    let mut test = DoubletsTest::new("test.links".to_string());

    test.run();
}

fn main() {
    bench_doublets();
}

fn mai456456n() {
    let mut db = DoubletsDbContext::<usize, _>::new(HeapMem::new());
    let contents =
    [
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis malesuada blandit mauris nec bibendum. Phasellus feugiat vehicula mauris et aliquet. Integer et gravida velit, in rutrum leo. Duis pretium, nunc ac posuere porttitor, augue sapien commodo tortor, nec consequat lorem eros ultricies odio. Aliquam letius congue ex nec viverra. Pellentesque eu velit tellus. Donec ac luctus nisi. Curabitur dignissim sodales mauris eu semper. Ut pretium lorem nulla, sit amet auctor arcu placerat vitae. Quisque lacinia dolor et consectetur fermentum. Nam ac orci vitae nulla aliquam tempor ac a nibh. Ut ac tincidunt lacus. Morbi vitae felis lorem.".to_string(),
        "Curabitur tincidunt nibh sit amet finibus dictum. Suspendisse aliquet arcu non rutrum ultrices. Integer ullamcorper mauris sit amet nibh aliquam, et tempor turpis hendrerit. In molestie elit et mauris rutrum, non auctor ligula ultricies. Vestibulum dignissim mauris finibus libero interdum hendrerit. Nunc vitae ipsum porttitor, egestas magna ut, sagittis sem. Donec euismod ac tortor vel porta. Vivamus convallis, ex at vestibulum rutrum, velit purus venenatis metus, sit amet aliquam sapien nibh quis elit. Aenean id neque a orci sodales venenatis. Integer ut orci ligula. Interdum et malesuada fames ac ante ipsum primis in faucibus. Praesent molestie dolor non lobortis ornare. Duis quis nisl sollicitudin, accumsan ante sed, eleifend velit. Maecenas maximus sed ante nec auctor.".to_string(),
        "Donec vitae felis lectus. Aenean velit sapien, porttitor ut feugiat a, consectetur et risus. Proin ac viverra sem. Nullam sagittis ex tortor, eu pellentesque tellus efficitur at. Nunc non egestas leo. Nam sed suscipit neque. Nam sodales vel neque eget eleifend. Vivamus in condimentum elit, consectetur commodo ex. Suspendisse rutrum, sapien efficitur cursus sodales, dolor orci pulvinar mauris, eu fringilla leo ex id leo. Interdum et malesuada fames ac ante ipsum primis in faucibus. Proin rhoncus sapien massa, molestie vestibulum augue hendrerit nec. Aliquam malesuada letius sapien id accumsan. Duis blandit aliquet felis, nec pellentesque lacus tincidunt et. Cras sed ligula vel nisl laoreet sagittis. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Praesent tristique a neque aliquet aliquam.".to_string(),
        "Aliquam sed egestas felis. Maecenas sollicitudin nisl in sapien posuere vulputate. Suspendisse eleifend sem magna, interdum consectetur augue venenatis at. Vivamus ornare orci vel orci sodales maximus. Donec ultricies felis ac nulla fermentum gravida. Phasellus vulputate turpis odio, a letius nibh luctus et. Aliquam tincidunt, metus ut congue porttitor, nibh dui ullamcorper quam, a eleifend elit ipsum sit amet quam. Aenean venenatis mollis interdum. Nunc cursus ex sit amet enim lacinia hendrerit. Nullam at libero iaculis, consectetur velit in, porta sem. Ut mattis ut ex in imperdiet. Maecenas pellentesque sit amet dui eget vehicula. Sed posuere, arcu pretium convallis tincidunt, turpis leo dignissim felis, non euismod diam magna a risus. Suspendisse a arcu nec turpis pulvinar ullamcorper. Nunc iaculis malesuada elit eu pretium. Aenean a neque a sapien tincidunt faucibus.".to_string(),
        "Ut a eleifend augue, eget posuere augue. Proin purus neque, pretium condimentum ipsum ut, venenatis tincidunt nunc. In vitae odio in justo pharetra tincidunt. Maecenas vel tellus interdum, suscipit tellus sit amet, cursus justo. Mauris sollicitudin euismod molestie. Cras eros nisi, molestie vel elementum ut, consequat ac nunc. In consectetur nulla vitae interdum elementum. Praesent faucibus magna et iaculis congue. Curabitur convallis cursus porttitor. Praesent hendrerit justo ut sem convallis sollicitudin eu at odio.".to_string(),
    ];


    let instant = Instant::now();

    for _ in 0..10000 {
        db.push_string(&contents[rand::thread_rng().gen_range(0..5)]);
    }

    println!("{}", db.links.count());
    println!("{:?}", instant.elapsed());
}

fn main4() {
    std::fs::remove_file("_.links");

    {

        let mem = FileMappedMem::new("_.links").unwrap();
        let mut db = DoubletsDbContext::<usize, _>::new(mem);

        BlogPosts::generate(3000);

        for post in unsafe { &POSTS }.into_iter().take(100) {
            //println!("load");
            db.save_post(post.clone());
        }

        db.links.each(|link| {
            println!("{}", link);
            db.links.constants.r#continue
        });
    }

    let mem = FileMappedMem::new("_.links").unwrap();
    let mut db = DoubletsDbContext::<usize, _>::new(mem);

    println!("{}", db.meaning_root);
    println!("{}", db.unicode_marker);
    println!("{}", db.sequence_marker);
    println!("{}", db.title_marker);
    println!("{}", db.content_marker);
    println!("{}", db.publication_time_marker);
    println!("{}", db.blog_post_marker);

    println!("{:#?}", db.links.count());
    println!("{:#?}", db.blog_posts());

    db.links.each(|link| {
        println!("{}", link);
        db.links.constants.r#continue
    });
}

fn ma4in() {
    let mem = HeapMem::new();
    let mut db = DoubletsDbContext::<usize, _>::new(mem);

    let post = db.save_post(BlogPost {
        id: 12,
        title: "123".to_string(),
        content: "1234".to_string(),
        date: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
    });

    let post = db.save_post(BlogPost {
        id: 13,
        title: "4234234".to_string(),
        content: "3214".to_string(),
        date: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
    });

    println!("{:?}", db.blog_posts());
}

fn __main() {
    let mem = HeapMem::new();
    let mut db = DoubletsDbContext::<usize, _>::new(mem);

    let instant = Instant::now();

    for _ in 0..1000 {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(1000)
            .map(char::from)
            .collect();

        let index = db.push_string(&s);
        //db.read_string(index);
    }

    println!("{:?}", instant.elapsed());
    println!("{}", db.links.count());
}

fn _main() {
    let mem = HeapMem::new();
    let mut links = Links::<usize, _>::with_constants(mem, LinksConstants::via_only_external(true));

    let unicode_marker = links.create_point();
    let sequence_marker = links.create_point();

    let to_unicode = CharToUnicode::new(unicode_marker);
    let mut to_strseq = StringToUnicode::new(to_unicode, sequence_marker);

    let string = "MAMA LOVE PAPA И МЕНЯ".to_string();
    let str_link = to_strseq.convert(&mut links, &string);

    let to_char = UnicodeToChar::new(unicode_marker);
    let mut to_string = UnicodeToString::new(to_char, sequence_marker);

    let new_string = to_string.convert(&mut links, str_link);

    println!("{:?}", new_string);

    let r#continue = links.constants().r#continue;
    links.each(|link| {
        println!("{}->{}", link.source, link.target);
        r#continue
    });
}
