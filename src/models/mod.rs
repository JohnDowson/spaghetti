#![allow(dead_code)]
///placeholder
struct Time;
enum Update {
    Content(String),
    Title(String),
}
pub struct BlogPost {
    title: String,
    content: String,
    create_date: Time,
    edited: bool,
    last_edit_date: Time,
}
impl BlogPost {
    fn get() -> BlogPost {
        BlogPost {
            title: String::default(),
            content: String::default(),
            create_date: Time,
            edited: bool::default(),
            last_edit_date: Time,
        }
    }
    fn update(update: Update) -> Result<(), &'static str> {
        match update {
            Update::Content(_c) => {
                /* update post body */
                Ok(())
            }
            Update::Title(_t) => {
                /* update post title */
                Ok(())
            }
        }
    }
}
