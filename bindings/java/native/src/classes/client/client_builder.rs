// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{client::BrokerOptions as RustBrokerOptions, Api, ClientBuilder as RustClientBuilder};
use std::{cell::RefCell, convert::TryFrom, rc::Rc, time::Duration};

use crate::{full_node_api::Client, Result};

pub struct BrokerOptions {
    builder: Rc<RefCell<Option<RustBrokerOptions>>>,
}

impl BrokerOptions {
    pub fn new() -> Self {
        Self {
            builder: Rc::new(RefCell::new(Option::from(RustBrokerOptions::default()))),
        }
    }

    fn new_with(options: RustBrokerOptions) -> BrokerOptions {
        Self {
            builder: Rc::new(RefCell::new(Option::from(options))),
        }
    }

    pub fn automatic_disconnect(&self, disconnect: bool) -> BrokerOptions {
        let builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .automatic_disconnect(disconnect);
        BrokerOptions::new_with(builder)
    }

    pub fn timeout(&self, timeout: Duration) -> BrokerOptions {
        let builder = self.builder.borrow_mut().take().unwrap().timeout(timeout);
        BrokerOptions::new_with(builder)
    }

    pub fn use_ws(&self, use_ws: bool) -> BrokerOptions {
        let builder = self.builder.borrow_mut().take().unwrap().use_ws(use_ws);
        BrokerOptions::new_with(builder)
    }

    pub fn port(&self, port: u16) -> BrokerOptions {
        let builder = self.builder.borrow_mut().take().unwrap().port(port);
        BrokerOptions::new_with(builder)
    }

    pub fn max_reconnection_attempts(&self, max_reconnection_attempts: usize) -> BrokerOptions {
        let builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .max_reconnection_attempts(max_reconnection_attempts);
        BrokerOptions::new_with(builder)
    }
}

pub struct ClientBuilder {
    builder: Rc<RefCell<Option<RustClientBuilder>>>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            builder: Rc::new(RefCell::new(Option::from(RustClientBuilder::default()))),
        }
    }
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    fn new_with_builder(builder: RustClientBuilder) -> Self {
        Self {
            builder: Rc::new(RefCell::new(Option::from(builder))),
        }
    }

    pub fn with_node(&mut self, node: &str) -> ClientBuilder {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_node(node).unwrap();
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_nodes(&mut self, nodes: Vec<String>) -> ClientBuilder {
        let nodes_arr: Vec<&str> = nodes
            .iter()
            .map(|s| {
                let st: &str = &s;
                st
            })
            .collect();
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_nodes(nodes_arr.as_slice())
            .unwrap();
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_node_auth(
        &mut self,
        node: &str,
        jwt: Option<&str>,
        username: Option<&str>,
        password: Option<&str>,
    ) -> ClientBuilder {
        let jwt_opt = match jwt {
            Some(j) => Some(j.to_string()),
            None => None,
        };
        let auth_opt = match username {
            Some(user) => Some((user, password.unwrap())),
            None => None,
        };
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_node_auth(node, jwt_opt, auth_opt)
            .unwrap();
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_primary_node(
        &mut self,
        node: &str,
        jwt: Option<&str>,
        username: Option<&str>,
        password: Option<&str>,
    ) -> ClientBuilder {
        let jwt_opt = match jwt {
            Some(j) => Some(j.to_string()),
            None => None,
        };
        let auth_opt = match username {
            Some(user) => Some((user, password.unwrap())),
            None => None,
        };
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_primary_node(node, jwt_opt, auth_opt)
            .unwrap();
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_primary_pow_node(
        &mut self,
        node: &str,
        jwt: Option<&str>,
        username: Option<&str>,
        password: Option<&str>,
    ) -> ClientBuilder {
        let jwt_opt = match jwt {
            Some(j) => Some(j.to_string()),
            None => None,
        };
        let auth_opt = match username {
            Some(user) => Some((user, password.unwrap())),
            None => None,
        };
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_node_auth(node, jwt_opt, auth_opt)
            .unwrap();
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_permanode(
        &mut self,
        node: &str,
        jwt: Option<&str>,
        username: Option<&str>,
        password: Option<&str>,
    ) -> ClientBuilder {
        let jwt_opt = match jwt {
            Some(j) => Some(j.to_string()),
            None => None,
        };
        let auth_opt = match username {
            Some(user) => Some((user, password.unwrap())),
            None => None,
        };
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_permanode(node, jwt_opt, auth_opt)
            .unwrap();
        ClientBuilder::new_with_builder(new_builder)
    }
    
    /// Allows creating the client without nodes for offline address generation or signing
    pub fn with_offline_mode(&mut self) -> Self {
        let new_builder = crate::block_on(async move {
            self.builder
                .borrow_mut()
                .take()
                .unwrap()
                .with_offline_mode()
        });

        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_node_pool_urls(&mut self, node_pool_urls: Vec<String>) -> ClientBuilder {
        let new_builder = crate::block_on(async move {
            self.builder
                .borrow_mut()
                .take()
                .unwrap()
                .with_node_pool_urls(&node_pool_urls)
                .await
                .unwrap()
        });

        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_network(&mut self, network: String) -> ClientBuilder {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_network(&network);
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_node_sync_interval(&mut self, node_sync_interval: Duration) -> ClientBuilder {
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_node_sync_interval(node_sync_interval);
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_node_sync_disabled(&mut self) -> ClientBuilder {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_node_sync_disabled();
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_quorum(&self, quorum: bool) -> ClientBuilder {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_quorum(quorum);
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_quorum_size(&self, quorum_size: usize) -> ClientBuilder {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_quorum_size(quorum_size);
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_quorum_threshold(&self, quorum_size: usize) -> ClientBuilder {
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_quorum_threshold(quorum_size);
        ClientBuilder::new_with_builder(new_builder)
    }

    /// Sets the MQTT broker options.
    pub fn with_mqtt_broker_options(&mut self, options: BrokerOptions) -> ClientBuilder {
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_mqtt_broker_options(options.builder.borrow_mut().take().unwrap());
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_local_pow(&mut self, local: bool) -> ClientBuilder {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_local_pow(local);
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_tips_interval(&mut self, tips: u64) -> ClientBuilder {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_tips_interval(tips);
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_request_timeout(&mut self, timeout: Duration) -> ClientBuilder {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_request_timeout(timeout);
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_api_timeout(&mut self, api: Api, timeout: Duration) -> ClientBuilder {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_api_timeout(api, timeout);
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn finish(&mut self) -> Result<Client> {
        let client = crate::block_on(async move { self.builder.borrow_mut().take().unwrap().finish().await.unwrap() });

        Ok(Client::try_from(client).unwrap())
    }
}
