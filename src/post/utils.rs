
use serde_json::json;
use serde_json::Value;

pub fn create_data(
    text_description: String,
    person_id: String,
    media_description:String,
    media_id: String,
) -> Value {
    let form = if media_id.is_empty() {
         json!({
                  "author": person_id,
                  "commentary": &text_description,
                  "visibility": "PUBLIC",
                  "distribution": {
                    "feedDistribution": "MAIN_FEED",
                    "thirdPartyDistributionChannels": []
                  },
                  "content":{},
                  "lifecycleState": "PUBLISHED",
                  "isReshareDisabledByAuthor": false
           })
    } else {
        json!({
                "author": person_id,
                  "commentary": &text_description,
                  "visibility": "PUBLIC",
                  "distribution": {
                    "feedDistribution": "MAIN_FEED",
                    "targetEntities": [{
                                "geoLocations": [

                                ],
                                "seniorities": [

                                ]
                            }],
                    "thirdPartyDistributionChannels": []
                  },
                  "content":{
                    "media":{
                      "title":&media_description,
                      "id": &media_id
                      }
                    },
                  "lifecycleState": "PUBLISHED",
                  "isReshareDisabledByAuthor": true
           })
    };
    form
}