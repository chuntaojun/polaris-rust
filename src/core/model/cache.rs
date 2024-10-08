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

use crate::core::model::error::PolarisError;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EventType {
    Unknown,
    Instance,
    RouterRule,
    CircuitBreakerRule,
    RateLimitRule,
    Service,
    FaultDetectRule,
    ServiceContract,
    LaneRule,
    Namespaces,
    ConfigFile,
    ConfigGroup,
}

pub struct ServerEvent {
    pub event_key: ResourceEventKey,
    pub err: PolarisError,
    pub value: Box<dyn RegistryCacheValue>,
}

pub struct ResourceEventKey {
    pub namespace: String,
    pub event_type: EventType,
    pub filter: HashMap<String, String>,
}

pub trait RegistryCacheValue {
    fn is_loaded_from_file(&self) -> bool;

    fn event_type(&self) -> EventType;

    fn is_initialized(&self) -> bool;

    fn revision(&self) -> String;
}
