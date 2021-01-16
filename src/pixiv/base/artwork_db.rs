#[macro_export]
macro_rules! field_map {
    (ID) => {
        "_id"
    };
    (ArtworkID) => {
        "id"
    };
    (Caption) => {
        "caption"
    };
    (CreateDate) => {
        "create_date"
    };
    (ArtworkType) => {
        "type"
    };
    (Height) => {
        "height"
    };
    (Width) => {
        "width"
    };
    (SanityLevel) => {
        "sanity_level"
    };
    (Title) => {
        "title"
    };
    (TotalBookmarks) => {
        "total_bookmarks"
    };
    (TotalView) => {
        "total_view"
    };
    (ImageUrls) => {
        "image_urls"
    };
    (LastUpdateTime) => {
        "last_update_time"
    };
    (User) => {
        "user"
    };
    (Tags) => {
        "tags"
    };
    (ImageUrlMedia) => {
        "medium"
    };
    (ImageUrlSquareMedium) => {
        "square_medium"
    };
    (ImageUrlLarge) => {
        "large"
    };
    (UserAccount) => {
        "account"
    };
    (UserID) => {
        "id"
    };
    (UserName) => {
        "name"
    };
    (TagName) => {
        "name"
    };
    (TagTrans) => {
        "translated_name"
    };
}
#[macro_export]
macro_rules! field_mapi {
    (ID) => {
        "_id"
    };
    (ArtworkID) => {
        "id"
    };
    (Caption) => {
        "caption"
    };
    (CreateDate) => {
        "create_date"
    };
    (ArtworkType) => {
        "type"
    };
    (Height) => {
        "height"
    };
    (Width) => {
        "width"
    };
    (SanityLevel) => {
        "sanity_level"
    };
    (Title) => {
        "title"
    };
    (TotalBookmarks) => {
        "total_bookmarks"
    };
    (TotalView) => {
        "total_view"
    };
    (ImageUrls) => {
        "image_urls"
    };
    (LastUpdateTime) => {
        "last_update_time"
    };
    (User) => {
        "user"
    };
    (Tags) => {
        "tags"
    };
    (ImageUrlMedia) => {
        "image_urls.medium"
    };
    (ImageUrlSquareMedium) => {
        "image_urls.square_medium"
    };
    (ImageUrlLarge) => {
        "image_urls.large"
    };
    (UserAccount) => {
        "user.account"
    };
    (UserID) => {
        "user.id"
    };
    (UserName) => {
        "user.name"
    };
    (TagName) => {
        "tags.name"
    };
    (TagTrans) => {
        "tags.translated_name"
    };
}

// macro_rules! put_value {
//     ($doc : expr,$f : expr,$v : expr) => {
//         if $v.is_some() {
//             $doc.insert($f.as_stri(),$v.as_ref().unwrap());
//         }
//     };
// }

// pub enum ArtworkField {
//     ID,
//     ArtworkID,
//     Caption,
//     CreateDate,
//     ArtworkType,
//     Height,
//     Width,
//     SanityLevel,
//     Title,
//     TotalBookmarks,
//     TotalView,
//     ImageUrls,
//     LastUpdateTime,
//     User,
//     Tags,
//     ImageUrlMedia,
//     ImageUrlSquareMedium,
//     ImageUrlLarge,
//     UserAccount,
//     UserID,
//     UserName,
//     TagName,
//     TagTrans,
// }
// impl ArtworkField {
//     pub fn as_stri(&self) -> &str {
//         match self {
//             ID => "_id",
//             ArtworkID => "id",
//             Caption => "caption",
//             CreateDate => "create_date",
//             ArtworkType => "type",
//             Height => "height",
//             Width => "width",
//             SanityLevel => "sanity_level",
//             Title => "title",
//             TotalBookmarks => "total_bookmarks",
//             TotalView => "total_view",
//             ImageUrls => "image_urls",
//             LastUpdateTime => "last_update_time",
//             User => "user",
//             Tags => "tags",
//             ImageUrlMedia => "medium",
//             ImageUrlSquareMedium => "square_medium",
//             ImageUrlLarge => "large",
//             UserAccount => "account",
//             UserID => "id",
//             UserName => "name",
//             TagName => "name",
//             TagTrans => "translated_name"
//         }
//     }

//     pub fn as_strq(&self) -> &str {
//         match self {
//             ID => "_id",
//             ArtworkID => "id",
//             Caption => "caption",
//             CreateDate => "create_date",
//             ArtworkType => "type",
//             Height => "height",
//             Width => "width",
//             SanityLevel => "sanity_level",
//             Title => "title",
//             TotalBookmarks => "total_bookmarks",
//             TotalView => "total_view",
//             ImageUrls => "image_urls",
//             LastUpdateTime => "last_update_time",
//             User => "user",
//             Tags => "tags",
//             ImageUrlMedia => "image_urls.medium",
//             ImageUrlSquareMedium => "image_urls.square_medium",
//             ImageUrlLarge => "image_urls.large",
//             UserAccount => "user.account",
//             UserID => "user.id",
//             UserName => "user.name",
//             TagName => "tags.name",
//             TagTrans => "tags.translated_name"
//         }
//     }
// }

// impl From<Artwork> for Document {
//     fn from(v: Artwork) -> Document {
//         let mut document = Document::new();
//         put_value!(document,ArtworkField::ID,v._id);
//         document.insert(ArtworkField::ArtworkID.as_stri(), v.artwork_id);
//         put_value!(document,ArtworkField::Caption,v.caption);
//         put_value!(document,ArtworkField::CreateDate,v.create_date);
//         put_value!(document,ArtworkField::ArtworkType,v.artwork_type);
//         put_value!(document,ArtworkField::Height,v.height);
//         put_value!(document,ArtworkField::Width,v.width);
//         put_value!(document,ArtworkField::SanityLevel,v.sanity_level);
//         put_value!(document,ArtworkField::Title,v.title);
//         put_value!(document,ArtworkField::TotalBookmarks,v.total_bookmarks);
//         put_value!(document,ArtworkField::TotalView,v.total_view);
//         put_value!(document,ArtworkField::LastUpdateTime,v.last_update_time);
//         if v.user.is_some() {
//             let mut sub_doc = Document::new();
//             let sub = v.user.as_ref().unwrap();
//             put_value!(sub_doc,ArtworkField::UserName,sub.name);
//             put_value!(sub_doc,ArtworkField::UserID,sub.user_id);
//             put_value!(sub_doc,ArtworkField::UserAccount,sub.account);
//             document.insert(ArtworkField::User.as_stri(),sub_doc);
//         }

//         if v.tags.is_some() {
//             let mut tag_array = Array::new();
//             let tags = v.tags.as_ref().unwrap();
//             for tag in tags {
//                 let mut sub_doc = Document::new();
//                 sub_doc.insert(ArtworkField::TagName.as_stri(),&tag.name);
//                 put_value!(sub_doc,ArtworkField::TagTrans,tag.trans);
//                 // tag_array.
//             }
//         }
//         document
//     }
// }
