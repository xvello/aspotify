use crate::model::*;

macro_rules! inherit_user_simplified {
    ($(#[$attr:meta])* $name:ident { $($(#[$f_attr:meta])* $f_name:ident : $f_ty:ty,)* }) => {
        to_struct!($(#[$attr])* $name {
            $(
                $(#[$f_attr])*
                $f_name: $f_ty,
            )*
            /// The name of the user; can be not available.
            display_name: Option<String>,
            /// Known public external URLs for this user.
            external_urls: HashMap<String, String>,
            /// The [Spotify user
            /// ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the
            /// user.
            id: String,
        });
    }
}

macro_rules! inherit_user_public {
    ($(#[$attr:meta])* $name:ident { $($(#[$f_attr:meta])* $f_name:ident : $f_ty:ty,)* }) => {
        inherit_user_simplified!($(#[$attr])* $name {
            $(
                $(#[$f_attr])*
                $f_name: $f_ty,
            )*
            /// Information about the followers of the user.
            followers: Followers,
            /// The user's profile image.
            images: Vec<Image>,
        });
    }
}

inherit_user_simplified!(
    /// A user object that contains less fields than UserPubic and is not documented anywhere, but
    /// is returned by some endpoints.
    UserSimplified {}
);
inherit_user_public!(
    /// A user object that is accessible to everyone.
    UserPublic {}
);
inherit_user_public!(
    /// A user object only accessible to the user themselves; does not work with Client Credentials
    /// flow.
    UserPrivate {
        /// The country of the user, as set in their account profile. Requires `user-read-private`.
        country: Option<CountryCode>,
        /// The user's email address, which is not necessarily a real email address. Requires
        /// `user-read-email`.
        email: Option<String>,
        /// The user's Spotify subscription level. Requires `user-read-private`.
        product: Option<Subscription>,
    }
);

/// The subscription level; premium or free.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Subscription {
    Premium,
    #[serde(alias = "open")]
    Free,
}
