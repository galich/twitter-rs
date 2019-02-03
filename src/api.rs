use std::collections::HashMap;

/// Collections/entries request
///
/// # Resource Information
///
/// Response formats: **JSON**
///
/// Requires authentication?: **Yes (user context only)**
///
/// Rate limited?: **Yes**
///
/// Requests / 15-min window (user auth): **1000**
///
/// [Docs on Twitter](https://developer.twitter.com/en/docs/tweets/curate-a-collection/api-reference/get-collections-entries)
///
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EntriesRequest {
    /// The identifier of the Collection for which to return results.
    pub id: String,

    /// Specifies the maximum number of results to include in the response.
    /// Specify a count between 1 and 200. A next_cursor value will be provided
    /// in the response if additional results are available.
    pub count: Option<u8>,

    /// Returns results with a position value less than or equal to the specified position.
    pub max_position: Option<u64>,

    /// Returns results with a position greater than the specified position.
    pub min_position: Option<u64>,
}

/// Collections/entries response
///
/// [Docs on Twitter](https://developer.twitter.com/en/docs/tweets/curate-a-collection/api-reference/get-collections-entries)
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EntriesResponse {
    pub objects: EntryObjects,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EntryObjects {
    pub timelines: HashMap<String, Timeline>,
    pub tweets: HashMap<String, Tweet>,
    pub response: EntryObjectsResponse,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Timeline {
    pub collection_type: String,
    pub collection_url: String,
    pub description: String,
    pub name: String,
    pub timeline_order: String,
    pub url: String,
    pub user_id: String,
    pub visibility: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Tweet {
    pub contributors: Option<Vec<Contributor>>,
    pub coordinates: Option<Vec<Coordinate>>,
    pub created_at: String, //@sg actually this is date/time
    pub entities: TweetEntities,
    pub extended_entities: TweetExtendedEntities,
    pub favorite_count: u64,
    pub favorited: bool,
    pub geo: Option<Geo>,
    pub id: u64,
    pub id_str: String,
    pub in_reply_to_screen_name: Option<String>,
    pub in_reply_to_status_id: Option<u64>,
    pub in_reply_to_status_id_str: Option<String>,
    pub in_reply_to_user_id: Option<u64>,
    pub in_reply_to_user_id_str: Option<String>,
    pub is_quote_status: bool,
    pub lang: String,
    pub place: Option<Place>,
    pub possibly_sensitive: bool,
    pub retweet_count: u64,
    pub retweeted: bool,
    pub source: String,
    pub text: String,
    pub truncated: bool,
    pub user: User,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Contributor {
    //@sg todo
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Coordinate {
    //@sg todo
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TweetEntities {
    pub hashtags: Vec<String>,
    pub media: Vec<Media>,
    pub symbols: Vec<String>,
    pub urls: Vec<String>,
    pub user_mentions: Vec<UserMention>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TweetExtendedEntities {
    pub media: Vec<Media>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Geo {
    //@sg todo
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Place {
    //@sg todo
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Media {
    pub display_url: String,
    pub expanded_url: String,
    pub id: u64,
    pub id_str: String,
    pub indices: Vec<u64>,
    pub media_url: String,
    pub media_url_https: String,
    pub sizes: Sizes,
    #[serde(rename = "type")]
    pub media_type: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UserMention {
    pub id: u64,
    pub id_str: String, //@sg not sure we need to keep that
    pub indices: Vec<u64>,
    pub name: String,
    pub screen_name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct User {
    pub id: u64,
    pub id_str: String, //@sg not sure we need to keep that
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Size {
    pub h: u16,
    pub w: u16,
    pub resize: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Sizes {
    pub large: Size,
    pub medium: Size,
    pub small: Size,
    pub thumb: Size,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EntryObjectsResponse {
    pub position: EntryObjectsResponsePosition,
    pub timeline: Vec<EntryObjectsResponseTimeline>,
    pub timeline_id: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EntryObjectsResponseTimeline {
    pub feature_context: String,
    pub tweet: TweetReference,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TweetReference {
    pub id: String,
    pub sort_index: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EntryObjectsResponsePosition {
    pub max_position: String,
    pub min_position: String,
    pub was_truncated: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_collections_entries() {
        // copied from https://developer.twitter.com/en/docs/tweets/curate-a-collection/api-reference/get-collections-entries#example-response
        let json = r#"{
          "objects": {
            "timelines": {
              "custom-539487832448843776": {
                "collection_type": "user",
                "collection_url": "https://twitter.com/TwitterDev/timelines/539487832448843776",
                "description": "A collection of Tweets about National Parks in the United States.",
                "name": "National Park Tweets",
                "timeline_order": "curation_reverse_chron",
                "url": "",
                "user_id": "2244994945",
                "visibility": "public"
              }
            },
            "tweets": {
              "504032379045179393": {
                "contributors": null,
                "coordinates": null,
                "created_at": "Mon Aug 25 22:27:38 +0000 2014",
                "entities": {
                  "hashtags": [],
                  "media": [
                    {
                      "display_url": "pic.twitter.com/HtdvV0bPEu",
                      "expanded_url": "http://twitter.com/Interior/status/504032379045179393/photo/1",
                      "id": 504032378411446273,
                      "id_str": "504032378411446273",
                      "indices": [
                        99,
                        121
                      ],
                      "media_url": "http://pbs.twimg.com/media/Bv6uxxaCcAEjWHD.jpg",
                      "media_url_https": "https://pbs.twimg.com/media/Bv6uxxaCcAEjWHD.jpg",
                      "sizes": {
                        "large": {
                          "h": 695,
                          "resize": "fit",
                          "w": 1024
                        },
                        "medium": {
                          "h": 407,
                          "resize": "fit",
                          "w": 600
                        },
                        "small": {
                          "h": 230,
                          "resize": "fit",
                          "w": 340
                        },
                        "thumb": {
                          "h": 150,
                          "resize": "crop",
                          "w": 150
                        }
                      },
                      "type": "photo",
                      "url": "http://t.co/HtdvV0bPEu"
                    }
                  ],
                  "symbols": [],
                  "urls": [],
                  "user_mentions": [
                    {
                      "id": 66453289,
                      "id_str": "66453289",
                      "indices": [
                        47,
                        60
                      ],
                      "name": "Lake Clark NP&P",
                      "screen_name": "LakeClarkNPS"
                    }
                  ]
                },
                "extended_entities": {
                  "media": [
                    {
                      "display_url": "pic.twitter.com/HtdvV0bPEu",
                      "expanded_url": "http://twitter.com/Interior/status/504032379045179393/photo/1",
                      "id": 504032378411446273,
                      "id_str": "504032378411446273",
                      "indices": [
                        99,
                        121
                      ],
                      "media_url": "http://pbs.twimg.com/media/Bv6uxxaCcAEjWHD.jpg",
                      "media_url_https": "https://pbs.twimg.com/media/Bv6uxxaCcAEjWHD.jpg",
                      "sizes": {
                        "large": {
                          "h": 695,
                          "resize": "fit",
                          "w": 1024
                        },
                        "medium": {
                          "h": 407,
                          "resize": "fit",
                          "w": 600
                        },
                        "small": {
                          "h": 230,
                          "resize": "fit",
                          "w": 340
                        },
                        "thumb": {
                          "h": 150,
                          "resize": "crop",
                          "w": 150
                        }
                      },
                      "type": "photo",
                      "url": "http://t.co/HtdvV0bPEu"
                    }
                  ]
                },
                "favorite_count": 639,
                "favorited": false,
                "geo": null,
                "id": 504032379045179393,
                "id_str": "504032379045179393",
                "in_reply_to_screen_name": null,
                "in_reply_to_status_id": null,
                "in_reply_to_status_id_str": null,
                "in_reply_to_user_id": null,
                "in_reply_to_user_id_str": null,
                "is_quote_status": false,
                "lang": "en",
                "place": null,
                "possibly_sensitive": false,
                "retweet_count": 606,
                "retweeted": false,
                "source": "Twitter for iPhone",
                "text": "How about a grizzly bear waving for the camera @LakeClarkNPS to end the day? Photo: Kevin Dietrich http://t.co/HtdvV0bPEu",
                "truncated": false,
                "user": {
                  "id": 76348185,
                  "id_str": "76348185"
                }
              }
          },
          "response": {
            "position": {
              "max_position": "371578415352947200",
              "min_position": "371578380871797248",
              "was_truncated": false
            },
            "timeline": [
              {
                "feature_context": "HBgGY3VzdG9tFoCAktzo1NL8DgAA",
                "tweet": {
                  "id": "504032379045179393",
                  "sort_index": "371578415352947200"
                }
              },
              {
                "feature_context": "HBgGY3VzdG9tFoCAktzo1NL8DgAA",
                "tweet": {
                  "id": "532654992071852032",
                  "sort_index": "371578393139797760"
                }
              },
              {
                "feature_context": "HBgGY3VzdG9tFoCAktzo1NL8DgAA",
                "tweet": {
                  "id": "524573263163572224",
                  "sort_index": "371578380871797248"
                }
              }
            ],
            "timeline_id": "custom-539487832448843776"
          }
        }}"#;

        let response = serde_json::from_str(json).unwrap();
        let expected = EntriesResponse {
            objects: EntryObjects {
                timelines: [(
                    "custom-539487832448843776".into(),
                    Timeline {
                        collection_type: "user".to_string(),
                        collection_url:
                            "https://twitter.com/TwitterDev/timelines/539487832448843776"
                                .to_string(),
                        description:
                            "A collection of Tweets about National Parks in the United States."
                                .to_string(),
                        name: "National Park Tweets".to_string(),
                        timeline_order: "curation_reverse_chron".to_string(),
                        url: "".to_string(),
                        user_id: "2244994945".to_string(),
                        visibility: "public".to_string(),
                    },
                )]
                .iter()
                .cloned()
                .collect(),
                tweets: [(
                    "504032379045179393".to_string(),
                    Tweet {
                        contributors: None,
                        coordinates: None,
                        created_at: "Mon Aug 25 22:27:38 +0000 2014".to_string(),
                        entities: TweetEntities {
                            hashtags: Vec::new(),
                            media: vec![Media {
                                display_url: "pic.twitter.com/HtdvV0bPEu".to_string(),
                                expanded_url:
                                    "http://twitter.com/Interior/status/504032379045179393/photo/1"
                                        .to_string(),
                                id: 504032378411446273,
                                id_str: "504032378411446273".to_string(),
                                indices: vec![99, 121],
                                media_url: "http://pbs.twimg.com/media/Bv6uxxaCcAEjWHD.jpg"
                                    .to_string(),
                                media_url_https: "https://pbs.twimg.com/media/Bv6uxxaCcAEjWHD.jpg"
                                    .to_string(),
                                sizes: Sizes {
                                    large: Size {
                                        h: 695,
                                        resize: "fit".to_string(),
                                        w: 1024,
                                    },
                                    medium: Size {
                                        h: 407,
                                        resize: "fit".to_string(),
                                        w: 600,
                                    },
                                    small: Size {
                                        h: 230,
                                        resize: "fit".to_string(),
                                        w: 340,
                                    },
                                    thumb: Size {
                                        h: 150,
                                        resize: "crop".to_string(),
                                        w: 150,
                                    },
                                },
                                media_type: "photo".to_string(),
                                url: "http://t.co/HtdvV0bPEu".to_string(),
                            }],
                            symbols: Vec::new(),
                            urls: Vec::new(),
                            user_mentions: vec![UserMention {
                                id: 66453289,
                                id_str: "66453289".to_string(),
                                indices: vec![47, 60],
                                name: "Lake Clark NP&P".to_string(),
                                screen_name: "LakeClarkNPS".to_string(),
                            }],
                        },
                        extended_entities: TweetExtendedEntities {
                            media: vec![Media {
                                display_url: "pic.twitter.com/HtdvV0bPEu".to_string(),
                                expanded_url:
                                    "http://twitter.com/Interior/status/504032379045179393/photo/1"
                                        .to_string(),
                                id: 504032378411446273,
                                id_str: "504032378411446273".to_string(),
                                indices: vec![99, 121],
                                media_url: "http://pbs.twimg.com/media/Bv6uxxaCcAEjWHD.jpg"
                                    .to_string(),
                                media_url_https: "https://pbs.twimg.com/media/Bv6uxxaCcAEjWHD.jpg"
                                    .to_string(),
                                sizes: Sizes {
                                    large: Size {
                                        h: 695,
                                        resize: "fit".to_string(),
                                        w: 1024,
                                    },
                                    medium: Size {
                                        h: 407,
                                        resize: "fit".to_string(),
                                        w: 600,
                                    },
                                    small: Size {
                                        h: 230,
                                        resize: "fit".to_string(),
                                        w: 340,
                                    },
                                    thumb: Size {
                                        h: 150,
                                        resize: "crop".to_string(),
                                        w: 150,
                                    },
                                },
                                media_type: "photo".to_string(),
                                url: "http://t.co/HtdvV0bPEu".to_string(),
                            }],
                        },
                        favorite_count: 639,
                        favorited: false,
                        geo: None,
                        id: 504032379045179393,
                        id_str: "504032379045179393".to_string(),
                        in_reply_to_screen_name: None,
                        in_reply_to_status_id: None,
                        in_reply_to_status_id_str: None,
                        in_reply_to_user_id: None,
                        in_reply_to_user_id_str: None,
                        is_quote_status: false,
                        lang: "en".to_string(),
                        place: None,
                        possibly_sensitive: false,
                        retweet_count: 606,
                        retweeted: false,
                        source: "Twitter for iPhone".to_string(),
                        text: "How about a grizzly bear waving for the camera @LakeClarkNPS to \
                               end the day? Photo: Kevin Dietrich http://t.co/HtdvV0bPEu"
                            .to_string(),
                        truncated: false,
                        user: User {
                            id: 76348185,
                            id_str: "76348185".to_string(),
                        },
                    },
                )]
                .iter()
                .cloned()
                .collect(),
                response: EntryObjectsResponse {
                    position: EntryObjectsResponsePosition {
                        max_position: "371578415352947200".to_string(),
                        min_position: "371578380871797248".to_string(),
                        was_truncated: false,
                    },
                    timeline: vec![
                        EntryObjectsResponseTimeline {
                            feature_context: "HBgGY3VzdG9tFoCAktzo1NL8DgAA".to_string(),
                            tweet: TweetReference {
                                id: "504032379045179393".to_string(),
                                sort_index: "371578415352947200".to_string(),
                            },
                        },
                        EntryObjectsResponseTimeline {
                            feature_context: "HBgGY3VzdG9tFoCAktzo1NL8DgAA".to_string(),
                            tweet: TweetReference {
                                id: "532654992071852032".to_string(),
                                sort_index: "371578393139797760".to_string(),
                            },
                        },
                        EntryObjectsResponseTimeline {
                            feature_context: "HBgGY3VzdG9tFoCAktzo1NL8DgAA".to_string(),
                            tweet: TweetReference {
                                id: "524573263163572224".to_string(),
                                sort_index: "371578380871797248".to_string(),
                            },
                        },
                    ],
                    timeline_id: "custom-539487832448843776".to_string(),
                },
            },
        };

        assert_eq!(expected, response);
    }
}
