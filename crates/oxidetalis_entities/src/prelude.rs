// OxideTalis Messaging Protocol homeserver core implementation
// Copyright (C) 2024 Awiteb <a@4rs.nl>, OxideTalis Contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub use sea_orm::{
    ActiveModelTrait,
    ColumnTrait,
    EntityOrSelect,
    EntityTrait,
    IntoActiveModel,
    ModelTrait,
    Order,
    PaginatorTrait,
    QueryFilter,
    QueryOrder,
    QuerySelect,
    Set,
    SqlErr,
};

/// User ID type
pub type UserId = i64;

pub use super::incoming_chat_requests::{
    ActiveModel as InChatRequestsActiveModel,
    Column as InChatRequestsColumn,
    Entity as InChatRequestsEntity,
    Model as InChatRequestsModel,
};
pub use super::outgoing_chat_requests::{
    ActiveModel as OutChatRequestsActiveModel,
    Column as OutChatRequestsColumn,
    Entity as OutChatRequestsEntity,
    Model as OutChatRequestsModel,
};
pub use super::users::{
    ActiveModel as UserActiveModel,
    Column as UserColumn,
    Entity as UserEntity,
    Model as UserModel,
};
pub use super::users_status::{
    AccessStatus,
    ActiveModel as UsersStatusActiveModel,
    Column as UsersStatusColumn,
    Entity as UsersStatusEntity,
    Model as UsersStatusModel,
};
