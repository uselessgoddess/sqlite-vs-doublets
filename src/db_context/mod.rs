use std::collections::HashMap;
use std::default::default;
use std::marker::PhantomData;
use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use doublets::doublets::data::{AddrToRaw, LinksConstants, Point, RawToAddr};
use doublets::doublets::{ILinks, ILinksExtensions};
use doublets::doublets::mem::united::Links;
use doublets::mem::{HeapMem, ResizeableMem};
use doublets::num::LinkType;
use num_traits::{one, zero};
use rand::Rng;
use crate::sequences::unicode::{CharToUnicode, StringToUnicode, UnicodeToChar, UnicodeToString};

use cached::proc_macro::cached;

#[derive(Debug, Default, Clone)]
pub struct BlogPost {
    pub id: usize,
    pub title: String,
    pub content: String,
    pub date: Duration
}

pub static mut POSTS: Vec<BlogPost> = vec![];

pub struct BlogPosts;
impl BlogPosts {
    pub fn generate(records: usize) {
        let contents =
        [
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis malesuada blandit mauris nec bibendum. Phasellus feugiat vehicula mauris et aliquet. Integer et gravida velit, in rutrum leo. Duis pretium, nunc ac posuere porttitor, augue sapien commodo tortor, nec consequat lorem eros ultricies odio. Aliquam letius congue ex nec viverra. Pellentesque eu velit tellus. Donec ac luctus nisi. Curabitur dignissim sodales mauris eu semper. Ut pretium lorem nulla, sit amet auctor arcu placerat vitae. Quisque lacinia dolor et consectetur fermentum. Nam ac orci vitae nulla aliquam tempor ac a nibh. Ut ac tincidunt lacus. Morbi vitae felis lorem.",
            "Curabitur tincidunt nibh sit amet finibus dictum. Suspendisse aliquet arcu non rutrum ultrices. Integer ullamcorper mauris sit amet nibh aliquam, et tempor turpis hendrerit. In molestie elit et mauris rutrum, non auctor ligula ultricies. Vestibulum dignissim mauris finibus libero interdum hendrerit. Nunc vitae ipsum porttitor, egestas magna ut, sagittis sem. Donec euismod ac tortor vel porta. Vivamus convallis, ex at vestibulum rutrum, velit purus venenatis metus, sit amet aliquam sapien nibh quis elit. Aenean id neque a orci sodales venenatis. Integer ut orci ligula. Interdum et malesuada fames ac ante ipsum primis in faucibus. Praesent molestie dolor non lobortis ornare. Duis quis nisl sollicitudin, accumsan ante sed, eleifend velit. Maecenas maximus sed ante nec auctor.",
            "Donec vitae felis lectus. Aenean velit sapien, porttitor ut feugiat a, consectetur et risus. Proin ac viverra sem. Nullam sagittis ex tortor, eu pellentesque tellus efficitur at. Nunc non egestas leo. Nam sed suscipit neque. Nam sodales vel neque eget eleifend. Vivamus in condimentum elit, consectetur commodo ex. Suspendisse rutrum, sapien efficitur cursus sodales, dolor orci pulvinar mauris, eu fringilla leo ex id leo. Interdum et malesuada fames ac ante ipsum primis in faucibus. Proin rhoncus sapien massa, molestie vestibulum augue hendrerit nec. Aliquam malesuada letius sapien id accumsan. Duis blandit aliquet felis, nec pellentesque lacus tincidunt et. Cras sed ligula vel nisl laoreet sagittis. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Praesent tristique a neque aliquet aliquam.",
            "Aliquam sed egestas felis. Maecenas sollicitudin nisl in sapien posuere vulputate. Suspendisse eleifend sem magna, interdum consectetur augue venenatis at. Vivamus ornare orci vel orci sodales maximus. Donec ultricies felis ac nulla fermentum gravida. Phasellus vulputate turpis odio, a letius nibh luctus et. Aliquam tincidunt, metus ut congue porttitor, nibh dui ullamcorper quam, a eleifend elit ipsum sit amet quam. Aenean venenatis mollis interdum. Nunc cursus ex sit amet enim lacinia hendrerit. Nullam at libero iaculis, consectetur velit in, porta sem. Ut mattis ut ex in imperdiet. Maecenas pellentesque sit amet dui eget vehicula. Sed posuere, arcu pretium convallis tincidunt, turpis leo dignissim felis, non euismod diam magna a risus. Suspendisse a arcu nec turpis pulvinar ullamcorper. Nunc iaculis malesuada elit eu pretium. Aenean a neque a sapien tincidunt faucibus.",
            "Ut a eleifend augue, eget posuere augue. Proin purus neque, pretium condimentum ipsum ut, venenatis tincidunt nunc. In vitae odio in justo pharetra tincidunt. Maecenas vel tellus interdum, suscipit tellus sit amet, cursus justo. Mauris sollicitudin euismod molestie. Cras eros nisi, molestie vel elementum ut, consequat ac nunc. In consectetur nulla vitae interdum elementum. Praesent faucibus magna et iaculis congue. Curabitur convallis cursus porttitor. Praesent hendrerit justo ut sem convallis sollicitudin eu at odio."
        ];

        let mut vec = Vec::with_capacity(records);
        for id in 0..records {
            vec.push(BlogPost {
                id,
                title: format!("Blog post {}", id + 1),
                content: contents[rand::thread_rng().gen_range(0..5)].to_owned(),
                date: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            })
        }

        unsafe { POSTS = vec };
    }
}

pub struct PropertiesOperator<T: LinkType> {
    _phantom: PhantomData<T>
}

impl<T: LinkType> PropertiesOperator<T> {
    pub fn new() -> Self {
        Self { _phantom: PhantomData }
    }

    pub fn get<L: ILinks<T>>(&self, links: &L, object: T, property: T) -> T {
        let linked_property = links.search_or(object, property, default());
        if linked_property == default() {
            return default();
        }

        let constants = links.constants();
        let any = constants.any;
        let value = links.single([any, linked_property, any]);
        //println!("{:?}", value);
        if let Some(link) = value {
            link.target
        } else {
            default()
        }
    }

    pub fn set<L: ILinks<T>>(&self, links: &mut L, object: T, property: T, value: T) {
        let property = links.get_or_create(object, property);
        let any = links.constants().any;
        for index in links.all_indices([any, property]) {
            links.delete(index);
        }
        links.get_or_create(property, value);
    }
}

pub struct DoubletsDbContext<T: LinkType, M: ResizeableMem> {
    pub meaning_root: T,
    pub unicode_marker: T,
    pub sequence_marker: T,
    pub title_marker: T,
    pub content_marker: T,
    pub publication_time_marker: T,
    pub blog_post_marker: T,

     // string zone
    pub to_unicode: StringToUnicode<T>,
    pub from_unicode: UnicodeToString<T>,

    pub(crate) links: Links<T, M>,
}

impl<T: LinkType, M: ResizeableMem> DoubletsDbContext<T, M> {
    pub fn new(mem: M) -> Self {
        let constants = LinksConstants::via_only_external(true);
        let mut links = Links::<_, M>::with_constants(mem, constants);

        let mut mapping = one();
        let meaning_root = Self::get_or_create_meaning(&mut links, mapping); mapping = mapping + one();
        let unicode_marker = Self::get_or_create_mapping(&mut links, mapping, meaning_root); mapping = mapping + one();
        let sequence_marker = Self::get_or_create_mapping(&mut links, mapping, meaning_root); mapping = mapping + one();
        let title_marker = Self::get_or_create_mapping(&mut links, mapping, meaning_root); mapping = mapping + one();
        let content_marker = Self::get_or_create_mapping(&mut links, mapping, meaning_root); mapping = mapping + one();
        let publication_time_marker = Self::get_or_create_mapping(&mut links, mapping, meaning_root); mapping = mapping + one();
        let blog_post_marker = Self::get_or_create_mapping(&mut links, mapping, meaning_root); mapping = mapping + one();

        let to_unicode = CharToUnicode::new(unicode_marker);
        let to_unicode = StringToUnicode::new(to_unicode, sequence_marker);

        let to_char = UnicodeToChar::new(unicode_marker);
        let from_unicode = UnicodeToString::new(to_char, sequence_marker);

        Self {
            meaning_root,
            unicode_marker,
            sequence_marker,
            title_marker,
            content_marker,
            publication_time_marker,
            blog_post_marker,
            to_unicode,
            from_unicode,
            links,
        }
    }

    fn get_or_create_meaning<L: ILinks<T>>(links: &mut L, root: T) -> T {
        if links.exist(root) {
            root
        } else {
            links.create_point()
        }
    }

    fn get_or_create_mapping<L: ILinks<T>>(links: &mut L, mapping: T, root: T) -> T {
        if links.exist(mapping) {
            mapping
        } else {
            links.create_and_update(root, zero())
        }
    }

    pub fn delete(&mut self, index: T) {
        self.links.delete_usages(index);
        //self.links.update(index, zero(), zero());
        self.links.delete(index);
    }

    pub fn push_string(&mut self, string: &String) -> T {
        self.to_unicode.convert(&mut self.links, string)
    }

    pub fn read_string(&mut self, sequence: T) -> String {
        self.from_unicode.convert(&self.links, sequence)
    }

    pub fn blog_posts(&mut self) -> Vec<BlogPost>
    {
        let any = self.links.constants().any;
        let r#continue = self.links.constants().r#continue;
        //let count = self.links.count_by([any, self.blog_post_marker, any]);
       // let mut list = Vec::with_capacity(count.as_());
        let mut list = Vec::new();

        self.links.each_by(|link| {
            if Point::is_partial(&link) {
                list.push(link);
            }
            r#continue
        }, [any, self.blog_post_marker, any]);

        list.into_iter()
            .map(|link| self.load_post(link.index))
            .collect()
    }

    pub fn load_post(&mut self, post: T) -> BlogPost {
        //let links = &self.links;
        let property = PropertiesOperator::new();

        let id = post.as_();

        let title_marker = self.title_marker;
        let title_seq = property.get(&self.links, post, title_marker);
        let title = self.read_string(title_seq);

        let content_marker = self.content_marker;
        let content_seq = property.get(&self.links, post, content_marker);
        let content = self.read_string(content_seq);

        //let datetime_raw = property.get(links, post, self.publication_time_marker);
        //let datetime = RawToAddr::new().convert(datetime_raw).as_();
        //let date = Duration::from_nanos(datetime as u64);

        BlogPost {
            id,
            title,
            content,
            //date,
            date: Default::default()
        }
    }

    pub fn save_post(&mut self, post: BlogPost) -> T {
        let itself = self.links.constants.itself;
        let property = PropertiesOperator::new();

        let new = self.links.create_and_update(self.blog_post_marker, itself);
        let new = self.links.update(new, self.blog_post_marker, new);
        //let new = self.links.update(new, self.blog_post_marker, new);

        let title_seq = self.push_string(&post.title);
        //println!("title seq: {}", title_seq);
        property.set(&mut self.links, new, self.title_marker, title_seq);

        let content_seq = self.push_string(&post.content);
        property.set(&mut self.links, new, self.content_marker, content_seq);

        //property.set(&mut self.links, new, self.publication_time_marker, AddrToRaw::new().convert(T::from(post.date.as_nanos()).unwrap()));

        return new;
    }
}
