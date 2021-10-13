/// A nominatim client that is binded to the nominatim web api.
pub struct Client {
    /// The user agent of your service. This is required by the Nominatim
    /// terms of service.
    ///
    /// Note that changing it does nothing unless respecified in the client.
    pub user_agent: String,
    /// ***Strongly Recommended***, your email so Nominatim can contact you
    /// in case they dislike your usecase.
    pub email: Option<String>,
    /// The base URL
    pub base_url: reqwest::Url,
    pub client: reqwest::Client,
}

impl Client {
    /// Creates a new client base
    ///
    /// # User Agent
    ///
    /// The user agent of your service. This is required by the Nominatim
    /// terms of service.
    ///
    /// # Email
    ///
    /// ***Strongly Recommended***, your email so Nominatim can contact you
    /// in case they dislike your usecase.
    pub fn new(
        base_url: reqwest::Url,
        user_agent: String,
        email: Option<String>,
    ) -> reqwest::Result<Self> {
        Ok(Self {
            client: reqwest::Client::builder()
                .user_agent(&user_agent)
                .build()?,
            base_url,
            user_agent,
            email,
        })
    }
}
