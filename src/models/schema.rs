table! {
    post_tag_relations (post, tag) {
        post -> Nullable<Integer>,
        tag -> Nullable<Integer>,
    }
}

table! {
    posts (id) {
        id -> Integer,
        created -> Text,
        title -> Text,
        body -> Text,
        published -> Integer,
        views -> Integer,
    }
}

table! {
    tags (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(post_tag_relations -> posts (post));
joinable!(post_tag_relations -> tags (tag));

allow_tables_to_appear_in_same_query!(
    post_tag_relations,
    posts,
    tags,
);
