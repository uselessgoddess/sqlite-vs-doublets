use std::time::{Duration, Instant};
use doublets::doublets::{ILinks, ILinksExtensions};
use doublets::mem::{FileMappedMem, ResizeableMem};
use crate::db_context::{BlogPost, BlogPosts, DoubletsDbContext, POSTS};

fn measure<F: FnOnce()>(action: F) -> Duration {
    let instant = Instant::now();
    action();
    instant.elapsed()
}

pub struct DoubletsTest {
    filename: String,
    read_posts: Vec<BlogPost>,
}


impl DoubletsTest {
    pub fn new(filename: String) -> Self {
        Self { filename, read_posts: vec![] }
    }

    pub fn run(&mut self) {
        { std::fs::File::create(&self.filename).ok() };

        println!("prepare time: {:?}", measure(|| self.prepare()));
        println!("db size after prepare: {}", self.db_size());
        println!("list creation time: {:?}", measure(|| self.create_list()));
        println!("db size after creation: {}", self.db_size());
        println!("list reading time: {:?}", measure(|| self.read_list()));
        println!("db size after reading: {}", self.db_size());
        println!("total count: {}", self.links_count());
        println!("blogs len: {}", self.blogs_len());

        println!("list deleting time: {:?}", measure(|| self.delete_list()));
        println!("db size after deleting: {}", self.db_size());
        println!("total count: {}", self.links_count());

        // for post in (&self.read_posts).into_iter().take(100) {
        //     println!("{:?}", post);
        // }
    }

    pub fn blogs_len(&self) -> usize {
        let mem = FileMappedMem::new(&self.filename).unwrap();
        let mut db = DoubletsDbContext::<usize, _>::new(mem);

        //for blog in db.blog_posts().into_iter().take(1000) {
        //    println!("{:?}", blog);
        //}

        db.blog_posts().len()
    }

    pub fn links_count(&self) -> usize {
        let mem = FileMappedMem::new(&self.filename).unwrap();
        let db = DoubletsDbContext::<usize, _>::new(mem);

        db.links.count()
    }

    pub fn db_size(&self) -> usize {
        let file = std::fs::File::open(&self.filename).unwrap();
        file.metadata().unwrap().len() as usize
    }

    pub fn prepare(&self) {
        let mut mem = FileMappedMem::new(&self.filename).unwrap();
        let _db = DoubletsDbContext::<usize, _>::new(mem);
    }

    pub fn create_list(&self) {
        let mem = FileMappedMem::new(&self.filename).unwrap();
        let mut db = DoubletsDbContext::<usize, _>::new(mem);

        for post in unsafe { &POSTS } {
            db.save_post(post.clone());
        }
    }

    pub fn read_list(&mut self) {
        let mem = FileMappedMem::new(&self.filename).unwrap();
        let mut db = DoubletsDbContext::<usize, _>::new(mem);
        let posts = db.blog_posts();

        for post in posts {
            self.read_posts.push(post)
        }
    }

    fn delete_list(&mut self) {
        let mem = FileMappedMem::new(&self.filename).unwrap();
        let mut db = DoubletsDbContext::<usize, _>::new(mem);

        for post in db.blog_posts() {
            db.delete(post.id);
        }
    }

    fn delete_base(self) {
        std::fs::remove_file(self.filename).unwrap()
    }
}
