// Tencent is pleased to support the open source community by making Polaris available.
//
// Copyright (C) 2019 THL A29 Limited, a Tencent company. All rights reserved.
//
// Licensed under the BSD 3-Clause License (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://opensource.org/licenses/BSD-3-Clause
//
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use std::env;
use std::net::{IpAddr, ToSocketAddrs};
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use tokio::runtime::{Builder, Runtime};

use crate::core::config::config::Configuration;
use crate::core::config::global::DISCOVER_SERVER_CONNECTOR;
use crate::core::model::error::PolarisError;
use crate::core::model::naming::InstanceRequest;
use crate::core::plugin::plugins::{init_server_connector, Extensions, PluginContainer};
use crate::discovery::req::{
    InstanceDeregisterRequest, InstanceHeartbeatRequest, InstanceRegisterRequest,
    InstanceRegisterResponse,
};

use super::plugin::connector::Connector;

static SEQ: AtomicU64 = AtomicU64::new(1);

pub struct Engine
where
    Self: Send + Sync,
{
    runtime: Arc<Runtime>,

    server_connector: Arc<Box<dyn Connector>>,
}

impl Engine {
    pub fn new(conf: Configuration) -> Result<Self, PolarisError> {
        let runtime = Arc::new(
            Builder::new_multi_thread()
                .enable_all()
                .thread_name("polaris-client-thread-pool")
                .worker_threads(4)
                .build()
                .unwrap(),
        );

        let arc_conf = Arc::new(conf);
        let client_id = crate::core::plugin::plugins::acquire_client_id(arc_conf.clone());

        let mut containers = PluginContainer::default();
        // 初始化所有的插件
        let start_time = std::time::Instant::now();
        containers.register_all_plugin();
        tracing::info!("register_all_plugin cost: {:?}", start_time.elapsed());

        let connector_opt = &arc_conf.global.server_connectors;
        let extensions_ret = Extensions::build(client_id, arc_conf.clone(), runtime.clone());
        if extensions_ret.is_err() {
            return Err(extensions_ret.err().unwrap());
        }
        let arc_extensions = Arc::new(extensions_ret.unwrap());

        let init_connector_ret =
            init_server_connector(connector_opt, &containers, arc_extensions.clone());
        if init_connector_ret.is_err() {
            return Err(init_connector_ret.err().unwrap());
        }

        let connector_result = init_connector_ret.unwrap();
        let server_connector = connector_result;

        Ok(Self {
            runtime,
            server_connector: Arc::new(server_connector),
        })
    }

    /// sync_register_instance 同步注册实例
    pub async fn sync_register_instance(
        &self,
        req: InstanceRegisterRequest,
    ) -> Result<InstanceRegisterResponse, PolarisError> {
        let connector = self.server_connector.clone();
        let rsp = connector
            .register_instance(InstanceRequest {
                flow_id: {
                    let mut flow_id = req.flow_id.clone();
                    if flow_id.is_empty() {
                        flow_id = uuid::Uuid::new_v4().to_string();
                    }
                    flow_id
                },
                ttl: req.ttl.clone(),
                instance: req.convert_instance(),
            })
            .await;

        return match rsp {
            Ok(ins_rsp) => Ok(InstanceRegisterResponse {
                instance_id: ins_rsp.instance.id.clone(),
                exist: ins_rsp.exist.clone(),
            }),
            Err(err) => Err(err),
        };
    }

    /// sync_deregister_instance 同步注销实例
    pub async fn sync_deregister_instance(
        &self,
        req: InstanceDeregisterRequest,
    ) -> Result<(), PolarisError> {
        let connector = self.server_connector.clone();
        let rsp = connector
            .deregister_instance(InstanceRequest {
                flow_id: {
                    let mut flow_id = req.flow_id.clone();
                    if flow_id.is_empty() {
                        flow_id = uuid::Uuid::new_v4().to_string();
                    }
                    flow_id
                },
                ttl: 0,
                instance: req.convert_instance(),
            })
            .await;

        return match rsp {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        };
    }

    /// sync_instance_heartbeat 同步实例心跳
    pub async fn sync_instance_heartbeat(
        &self,
        req: InstanceHeartbeatRequest,
    ) -> Result<(), PolarisError> {
        let connector = self.server_connector.clone();
        let rsp = connector
            .heartbeat_instance(InstanceRequest {
                flow_id: {
                    let mut flow_id = req.flow_id.clone();
                    if flow_id.is_empty() {
                        flow_id = uuid::Uuid::new_v4().to_string();
                    }
                    flow_id
                },
                ttl: 0,
                instance: req.convert_instance(),
            })
            .await;

        return match rsp {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        };
    }

    pub fn get_executor(&self) -> Arc<Runtime> {
        self.runtime.clone()
    }
}
