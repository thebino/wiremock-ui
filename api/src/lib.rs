use crate::requests::state::State;
use crate::responses::mapping::Mapping;
use crate::responses::response::MappingResponse;
use crate::responses::scenario::ScenarioResponse;
use reqwest::{Client, StatusCode};
use responses::scenario::Scenario;

pub mod requests;
pub mod responses;

pub struct API {
    base_url: String,
    client: Client,
}

impl API {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
        }
    }

    /// Return all registered mappings
    pub async fn get_mappings(
        self,
    ) -> Result<Vec<Mapping>, Box<dyn std::error::Error + Send + Sync>> {
        let uri: String = format!("{}/__admin/mappings", self.base_url);
        let response: MappingResponse = self
            .client
            .get(uri)
            .send()
            .await?
            .json::<MappingResponse>()
            .await?;

        Ok(response.mappings)
    }

    /// Reset mappings to the default state and reset the request journal
    pub async fn reset(self) -> Result<StatusCode, Box<dyn std::error::Error + Send + Sync>> {
        let uri: String = format!("{}/__admin/reset", self.base_url);
        let response = self.client.post(uri).send().await?;
        Ok(response.status())
    }

    /// Shutdown the wiremock server
    pub async fn shutdown(self) -> Result<StatusCode, Box<dyn std::error::Error + Send + Sync>> {
        let uri: String = format!("{}/__admin/shutdown", self.base_url);
        let response = self.client.post(uri).send().await?;

        Ok(response.status())
    }

    /// Return all configured scenarios containing its name, current state and possible states
    pub async fn get_scenarios(
        self,
    ) -> Result<Vec<Scenario>, Box<dyn std::error::Error + Send + Sync>> {
        let uri: String = format!("{}/__admin/scenarios", self.base_url);
        let response = self
            .client
            .get(uri)
            .send()
            .await?
            .json::<ScenarioResponse>()
            .await?;

        Ok(response.scenarios)
    }

    /// Reset the state of all configured scenarios back to `Scenario.START`
    pub async fn reset_scenarios(
        self,
    ) -> Result<StatusCode, Box<dyn std::error::Error + Send + Sync>> {
        let uri: String = format!("{}/__admin/scenarios/reset", self.base_url);
        let response = self.client.post(uri).send().await?;

        Ok(response.status())
    }

    /// Reset the state of an individual scenario back to `Scenario.START`
    pub async fn reset_scenario(
        self,
        scenario_name: String,
    ) -> Result<StatusCode, Box<dyn std::error::Error + Send + Sync>> {
        let uri: String = format!(
            "{}/__admin/scenarios/{}/state",
            self.base_url, scenario_name
        );
        let response = self.client.post(uri).send().await?;

        Ok(response.status())
    }

    /// Set the state of an individual scenario to a specific value
    pub async fn set_scenario_state(
        self,
        scenario_name: String,
        scenario_state: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let uri: String = format!("{}/__admin/scenarios/{scenario_name},state", self.base_url);
        self.client
            .post(uri)
            .json(&State {
                state: scenario_state,
            })
            .send()
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::API;
    use httpmock::Method::{GET, POST};
    use httpmock::MockServer;
    use reqwest::StatusCode;
    use serde_json::json;

    #[tokio::test]
    async fn get_mappings_success() {
        // given
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/__admin/mappings");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                  "meta": {
                    "total": 2
                  },
                  "mappings": [
                    {
                      "id": "76ada7b0-49ae-4229-91c4-396a36f18e09",
                      "uuid": "76ada7b0-49ae-4229-91c4-396a36f18e09",
                      "request": {
                        "method": "GET",
                        "url": "/search?q=things",
                        "headers": {
                          "Accept": {
                            "equalTo": "application/json"
                          }
                        }
                      },
                      "response": {
                        "status": 200,
                        "jsonBody": [
                          "thing1",
                          "thing2"
                        ],
                        "headers": {
                          "Content-Type": "application/json"
                        }
                      }
                    },
                    {
                      "request": {
                        "method": "POST",
                        "urlPath": "/some/things",
                        "bodyPatterns": [
                          {
                            "equalToXml": "<stuff />"
                          }
                        ]
                      },
                      "response": {
                        "status": 201
                      }
                    }
                  ]
                }));
        });

        // when
        let api = API::new(server.base_url());
        let result = api.get_mappings().await;

        // then
        mock.assert_hits(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn get_mappings_fail() {
        // given
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/__admin/mappings");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                  "mappings": [
                    {
                      "response": {
                        "status": 201
                      }
                    }
                  ]
                }));
        });

        // when
        let api = API::new(server.base_url());
        let result = api.get_mappings().await;

        // then
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn reset_success() {
        // given
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/__admin/reset");
            then.status(200);
        });

        // when
        let api = API::new(server.base_url());
        let result = api.reset().await;

        // then
        mock.assert_hits(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), StatusCode::OK);
    }

    #[tokio::test]
    async fn reset_fail() {
        // given
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/__admin/reset");
            then.status(400);
        });

        // when
        let api = API::new(server.base_url());
        let result = api.reset().await;

        // then
        assert_eq!(result.unwrap(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn shutdown_success() {
        // given
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/__admin/shutdown");
            then.status(200).header("content-type", "application/json");
        });

        // when
        let api = API::new(server.base_url());
        let result = api.shutdown().await;

        // then
        mock.assert_hits(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), StatusCode::OK);
    }

    #[tokio::test]
    async fn shutdown_fail() {
        // given
        let server = MockServer::start();

        // when
        let api = API::new(server.base_url());
        let result = api.shutdown().await;

        // then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn get_scenarios_success() {
        // given
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/__admin/scenarios");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                      "scenarios": [
                        {
                          "id": "c8d249ec-d86d-48b1-88a8-a660e6848042",
                          "name": "my_scenario",
                          "possibleStates": [
                            "Started",
                            "state_1",
                            "state_2"
                          ],
                          "state": "state_2"
                        }
                      ]
                }));
        });

        // when
        let api = API::new(server.base_url());
        let result = api.get_scenarios().await;

        // then
        mock.assert_hits(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn reset_scenarios_success() {
        // given
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/__admin/scenarios/reset");
            then.status(200).header("content-type", "application/json");
        });

        // when
        let api = API::new(server.base_url());
        let result = api.reset_scenarios().await;

        // then
        mock.assert_hits(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), StatusCode::OK);
    }

    #[tokio::test]
    async fn reset_scenario_success() {
        // given
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/__admin/scenarios/1/state");
            then.status(200).header("content-type", "application/json");
        });

        // when
        let api = API::new(server.base_url());
        let result = api.reset_scenario("1".to_string()).await;

        // then
        mock.assert_hits(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), StatusCode::OK);
    }
}
