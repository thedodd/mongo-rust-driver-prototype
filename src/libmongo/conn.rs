/* Copyright 2013 10gen Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use extra::net::tcp::*;
use extra::future::*;

use util::*;

pub type PortResult = Result<~[u8], TcpErrData>;

/**
 * Trait for sockets used by Connection. Used as a traitobject.
 */
pub trait Socket {
    fn read_start(&self) -> Result<@Port<PortResult>, TcpErrData>;
    fn read_stop(&self) -> Result<(), TcpErrData>;
    fn write_future(&self, raw_write_data: ~[u8]) -> Future<Result<(), TcpErrData>>;
}

impl Socket for TcpSocket {
    fn read_start(&self) -> Result<@Port<PortResult>, TcpErrData> {
        self.read_start()
    }
    fn read_stop(&self) -> Result<(), TcpErrData> {
        self.read_stop()
    }
    fn write_future(&self, raw_write_data: ~[u8]) -> Future<Result<(), TcpErrData>> {
        self.write_future(raw_write_data)
    }
}
/**
 * Connection interface all connectors use (ReplicaSetConnection,
 * ShardedClusterConnection, NodeConnection).
 */
pub trait Connection {
    fn connect(&self) -> Result<(), MongoErr>;
    fn disconnect(&self) -> Result<(), MongoErr>;
    fn send(&self, data : ~[u8], read : bool) -> Result<(), MongoErr>;
    fn recv(&self) -> Result<~[u8], MongoErr>;
}
