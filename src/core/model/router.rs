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

use std::iter::Map;

use super::loadbalance::Criteria;

#[derive(Clone)]
enum MetadataFailoverType {
    MetadataFailoverNone,
    MetadataFailoverAll,
    MetadataFailoverNoKey,
}

enum TrafficLabel {
    Header,
    Cookie,
    Query,
    Method,
    Path,
    CallerIp,
}

pub struct Argument {
    pub traffic_label: TrafficLabel,
    pub key: String,
    pub value: String,
}

#[derive(Clone)]
pub struct CallerInfo {
    pub namespace: String,
    pub service: String,
    pub metadata: Map<String, String>,
    pub metadata_failover: MetadataFailoverType,
    pub criteria: Criteria,
}

pub struct CalleeInfo {
    pub traffic_labels: Vec<Argument>,
}
