// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.36.1
// 	protoc        v5.29.0
// source: ng_actor.proto

package proto

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type EntityChangeType int32

const (
	EntityChangeType_ENTITY_CHANGE_UNSET EntityChangeType = 0
	EntityChangeType_Create              EntityChangeType = 1
	EntityChangeType_Update              EntityChangeType = 2
	EntityChangeType_Delete              EntityChangeType = 3
)

// Enum value maps for EntityChangeType.
var (
	EntityChangeType_name = map[int32]string{
		0: "ENTITY_CHANGE_UNSET",
		1: "Create",
		2: "Update",
		3: "Delete",
	}
	EntityChangeType_value = map[string]int32{
		"ENTITY_CHANGE_UNSET": 0,
		"Create":              1,
		"Update":              2,
		"Delete":              3,
	}
)

func (x EntityChangeType) Enum() *EntityChangeType {
	p := new(EntityChangeType)
	*p = x
	return p
}

func (x EntityChangeType) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (EntityChangeType) Descriptor() protoreflect.EnumDescriptor {
	return file_ng_actor_proto_enumTypes[0].Descriptor()
}

func (EntityChangeType) Type() protoreflect.EnumType {
	return &file_ng_actor_proto_enumTypes[0]
}

func (x EntityChangeType) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use EntityChangeType.Descriptor instead.
func (EntityChangeType) EnumDescriptor() ([]byte, []int) {
	return file_ng_actor_proto_rawDescGZIP(), []int{0}
}

type LogoutType int32

const (
	LogoutType_LOGOUT_UNSET LogoutType = 0
	LogoutType_Self         LogoutType = 1
	LogoutType_Force        LogoutType = 2
)

// Enum value maps for LogoutType.
var (
	LogoutType_name = map[int32]string{
		0: "LOGOUT_UNSET",
		1: "Self",
		2: "Force",
	}
	LogoutType_value = map[string]int32{
		"LOGOUT_UNSET": 0,
		"Self":         1,
		"Force":        2,
	}
)

func (x LogoutType) Enum() *LogoutType {
	p := new(LogoutType)
	*p = x
	return p
}

func (x LogoutType) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (LogoutType) Descriptor() protoreflect.EnumDescriptor {
	return file_ng_actor_proto_enumTypes[1].Descriptor()
}

func (LogoutType) Type() protoreflect.EnumType {
	return &file_ng_actor_proto_enumTypes[1]
}

func (x LogoutType) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use LogoutType.Descriptor instead.
func (LogoutType) EnumDescriptor() ([]byte, []int) {
	return file_ng_actor_proto_rawDescGZIP(), []int{1}
}

type CacheEvictType int32

const (
	CacheEvictType_CACHE_EVICT_UNSET CacheEvictType = 0
	CacheEvictType_Normal            CacheEvictType = 1
	CacheEvictType_All               CacheEvictType = 2
)

// Enum value maps for CacheEvictType.
var (
	CacheEvictType_name = map[int32]string{
		0: "CACHE_EVICT_UNSET",
		1: "Normal",
		2: "All",
	}
	CacheEvictType_value = map[string]int32{
		"CACHE_EVICT_UNSET": 0,
		"Normal":            1,
		"All":               2,
	}
)

func (x CacheEvictType) Enum() *CacheEvictType {
	p := new(CacheEvictType)
	*p = x
	return p
}

func (x CacheEvictType) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (CacheEvictType) Descriptor() protoreflect.EnumDescriptor {
	return file_ng_actor_proto_enumTypes[2].Descriptor()
}

func (CacheEvictType) Type() protoreflect.EnumType {
	return &file_ng_actor_proto_enumTypes[2]
}

func (x CacheEvictType) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use CacheEvictType.Descriptor instead.
func (CacheEvictType) EnumDescriptor() ([]byte, []int) {
	return file_ng_actor_proto_rawDescGZIP(), []int{2}
}

type CacheEvictMsg struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	CacheType     int32                  `protobuf:"varint,1,opt,name=cacheType,proto3" json:"cacheType,omitempty"`
	EvictType     CacheEvictType         `protobuf:"varint,2,opt,name=evictType,proto3,enum=ng_proto.CacheEvictType" json:"evictType,omitempty"`
	Key           string                 `protobuf:"bytes,3,opt,name=key,proto3" json:"key,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *CacheEvictMsg) Reset() {
	*x = CacheEvictMsg{}
	mi := &file_ng_actor_proto_msgTypes[0]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *CacheEvictMsg) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CacheEvictMsg) ProtoMessage() {}

func (x *CacheEvictMsg) ProtoReflect() protoreflect.Message {
	mi := &file_ng_actor_proto_msgTypes[0]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use CacheEvictMsg.ProtoReflect.Descriptor instead.
func (*CacheEvictMsg) Descriptor() ([]byte, []int) {
	return file_ng_actor_proto_rawDescGZIP(), []int{0}
}

func (x *CacheEvictMsg) GetCacheType() int32 {
	if x != nil {
		return x.CacheType
	}
	return 0
}

func (x *CacheEvictMsg) GetEvictType() CacheEvictType {
	if x != nil {
		return x.EvictType
	}
	return CacheEvictType_CACHE_EVICT_UNSET
}

func (x *CacheEvictMsg) GetKey() string {
	if x != nil {
		return x.Key
	}
	return ""
}

type UserLogoutMsg struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	LogoutType    LogoutType             `protobuf:"varint,1,opt,name=logoutType,proto3,enum=ng_proto.LogoutType" json:"logoutType,omitempty"`
	AccessToken   string                 `protobuf:"bytes,2,opt,name=accessToken,proto3" json:"accessToken,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *UserLogoutMsg) Reset() {
	*x = UserLogoutMsg{}
	mi := &file_ng_actor_proto_msgTypes[1]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *UserLogoutMsg) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UserLogoutMsg) ProtoMessage() {}

func (x *UserLogoutMsg) ProtoReflect() protoreflect.Message {
	mi := &file_ng_actor_proto_msgTypes[1]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UserLogoutMsg.ProtoReflect.Descriptor instead.
func (*UserLogoutMsg) Descriptor() ([]byte, []int) {
	return file_ng_actor_proto_rawDescGZIP(), []int{1}
}

func (x *UserLogoutMsg) GetLogoutType() LogoutType {
	if x != nil {
		return x.LogoutType
	}
	return LogoutType_LOGOUT_UNSET
}

func (x *UserLogoutMsg) GetAccessToken() string {
	if x != nil {
		return x.AccessToken
	}
	return ""
}

type EntityChangedMsg struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	EntityType    string                 `protobuf:"bytes,1,opt,name=entityType,proto3" json:"entityType,omitempty"`
	ChangeType    EntityChangeType       `protobuf:"varint,2,opt,name=changeType,proto3,enum=ng_proto.EntityChangeType" json:"changeType,omitempty"`
	NewId         int64                  `protobuf:"varint,3,opt,name=newId,proto3" json:"newId,omitempty"`
	OldId         int64                  `protobuf:"varint,4,opt,name=oldId,proto3" json:"oldId,omitempty"`
	TenantId      int64                  `protobuf:"varint,5,opt,name=tenantId,proto3" json:"tenantId,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *EntityChangedMsg) Reset() {
	*x = EntityChangedMsg{}
	mi := &file_ng_actor_proto_msgTypes[2]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *EntityChangedMsg) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*EntityChangedMsg) ProtoMessage() {}

func (x *EntityChangedMsg) ProtoReflect() protoreflect.Message {
	mi := &file_ng_actor_proto_msgTypes[2]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use EntityChangedMsg.ProtoReflect.Descriptor instead.
func (*EntityChangedMsg) Descriptor() ([]byte, []int) {
	return file_ng_actor_proto_rawDescGZIP(), []int{2}
}

func (x *EntityChangedMsg) GetEntityType() string {
	if x != nil {
		return x.EntityType
	}
	return ""
}

func (x *EntityChangedMsg) GetChangeType() EntityChangeType {
	if x != nil {
		return x.ChangeType
	}
	return EntityChangeType_ENTITY_CHANGE_UNSET
}

func (x *EntityChangedMsg) GetNewId() int64 {
	if x != nil {
		return x.NewId
	}
	return 0
}

func (x *EntityChangedMsg) GetOldId() int64 {
	if x != nil {
		return x.OldId
	}
	return 0
}

func (x *EntityChangedMsg) GetTenantId() int64 {
	if x != nil {
		return x.TenantId
	}
	return 0
}

type DeviceSessionEventMsg struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	Type          SessionType            `protobuf:"varint,1,opt,name=type,proto3,enum=ng_proto.SessionType" json:"type,omitempty"`
	Event         SessionEvent           `protobuf:"varint,2,opt,name=event,proto3,enum=ng_proto.SessionEvent" json:"event,omitempty"`
	SessionInfo   *SessionInfoProto      `protobuf:"bytes,3,opt,name=sessionInfo,proto3" json:"sessionInfo,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *DeviceSessionEventMsg) Reset() {
	*x = DeviceSessionEventMsg{}
	mi := &file_ng_actor_proto_msgTypes[3]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *DeviceSessionEventMsg) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DeviceSessionEventMsg) ProtoMessage() {}

func (x *DeviceSessionEventMsg) ProtoReflect() protoreflect.Message {
	mi := &file_ng_actor_proto_msgTypes[3]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use DeviceSessionEventMsg.ProtoReflect.Descriptor instead.
func (*DeviceSessionEventMsg) Descriptor() ([]byte, []int) {
	return file_ng_actor_proto_rawDescGZIP(), []int{3}
}

func (x *DeviceSessionEventMsg) GetType() SessionType {
	if x != nil {
		return x.Type
	}
	return SessionType_SESSION_UNSET
}

func (x *DeviceSessionEventMsg) GetEvent() SessionEvent {
	if x != nil {
		return x.Event
	}
	return SessionEvent_SESSION_EVENT_UNSET
}

func (x *DeviceSessionEventMsg) GetSessionInfo() *SessionInfoProto {
	if x != nil {
		return x.SessionInfo
	}
	return nil
}

type DeviceDeletedMsg struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	TenantId      int64                  `protobuf:"varint,1,opt,name=tenantId,proto3" json:"tenantId,omitempty"`
	DeviceId      int64                  `protobuf:"varint,2,opt,name=deviceId,proto3" json:"deviceId,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *DeviceDeletedMsg) Reset() {
	*x = DeviceDeletedMsg{}
	mi := &file_ng_actor_proto_msgTypes[4]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *DeviceDeletedMsg) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DeviceDeletedMsg) ProtoMessage() {}

func (x *DeviceDeletedMsg) ProtoReflect() protoreflect.Message {
	mi := &file_ng_actor_proto_msgTypes[4]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use DeviceDeletedMsg.ProtoReflect.Descriptor instead.
func (*DeviceDeletedMsg) Descriptor() ([]byte, []int) {
	return file_ng_actor_proto_rawDescGZIP(), []int{4}
}

func (x *DeviceDeletedMsg) GetTenantId() int64 {
	if x != nil {
		return x.TenantId
	}
	return 0
}

func (x *DeviceDeletedMsg) GetDeviceId() int64 {
	if x != nil {
		return x.DeviceId
	}
	return 0
}

type HousekeeperTaskRequest struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	EntityType    string                 `protobuf:"bytes,1,opt,name=entityType,proto3" json:"entityType,omitempty"`
	EntityId      int64                  `protobuf:"varint,2,opt,name=entityId,proto3" json:"entityId,omitempty"`
	TenantId      int64                  `protobuf:"varint,3,opt,name=tenantId,proto3" json:"tenantId,omitempty"`
	Type          HousekeeperTaskType    `protobuf:"varint,4,opt,name=type,proto3,enum=ng_proto.HousekeeperTaskType" json:"type,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *HousekeeperTaskRequest) Reset() {
	*x = HousekeeperTaskRequest{}
	mi := &file_ng_actor_proto_msgTypes[5]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *HousekeeperTaskRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*HousekeeperTaskRequest) ProtoMessage() {}

func (x *HousekeeperTaskRequest) ProtoReflect() protoreflect.Message {
	mi := &file_ng_actor_proto_msgTypes[5]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use HousekeeperTaskRequest.ProtoReflect.Descriptor instead.
func (*HousekeeperTaskRequest) Descriptor() ([]byte, []int) {
	return file_ng_actor_proto_rawDescGZIP(), []int{5}
}

func (x *HousekeeperTaskRequest) GetEntityType() string {
	if x != nil {
		return x.EntityType
	}
	return ""
}

func (x *HousekeeperTaskRequest) GetEntityId() int64 {
	if x != nil {
		return x.EntityId
	}
	return 0
}

func (x *HousekeeperTaskRequest) GetTenantId() int64 {
	if x != nil {
		return x.TenantId
	}
	return 0
}

func (x *HousekeeperTaskRequest) GetType() HousekeeperTaskType {
	if x != nil {
		return x.Type
	}
	return HousekeeperTaskType_HOUSEKEEPER_TASK_UNSET
}

type HousekeeperTaskResponse struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	Code          int32                  `protobuf:"varint,1,opt,name=code,proto3" json:"code,omitempty"`
	Msg           string                 `protobuf:"bytes,2,opt,name=msg,proto3" json:"msg,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *HousekeeperTaskResponse) Reset() {
	*x = HousekeeperTaskResponse{}
	mi := &file_ng_actor_proto_msgTypes[6]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *HousekeeperTaskResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*HousekeeperTaskResponse) ProtoMessage() {}

func (x *HousekeeperTaskResponse) ProtoReflect() protoreflect.Message {
	mi := &file_ng_actor_proto_msgTypes[6]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use HousekeeperTaskResponse.ProtoReflect.Descriptor instead.
func (*HousekeeperTaskResponse) Descriptor() ([]byte, []int) {
	return file_ng_actor_proto_rawDescGZIP(), []int{6}
}

func (x *HousekeeperTaskResponse) GetCode() int32 {
	if x != nil {
		return x.Code
	}
	return 0
}

func (x *HousekeeperTaskResponse) GetMsg() string {
	if x != nil {
		return x.Msg
	}
	return ""
}

type DriverChangedMsg struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	TenantId      int64                  `protobuf:"varint,1,opt,name=tenantId,proto3" json:"tenantId,omitempty"`
	DriverId      int64                  `protobuf:"varint,2,opt,name=driverId,proto3" json:"driverId,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *DriverChangedMsg) Reset() {
	*x = DriverChangedMsg{}
	mi := &file_ng_actor_proto_msgTypes[7]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *DriverChangedMsg) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DriverChangedMsg) ProtoMessage() {}

func (x *DriverChangedMsg) ProtoReflect() protoreflect.Message {
	mi := &file_ng_actor_proto_msgTypes[7]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use DriverChangedMsg.ProtoReflect.Descriptor instead.
func (*DriverChangedMsg) Descriptor() ([]byte, []int) {
	return file_ng_actor_proto_rawDescGZIP(), []int{7}
}

func (x *DriverChangedMsg) GetTenantId() int64 {
	if x != nil {
		return x.TenantId
	}
	return 0
}

func (x *DriverChangedMsg) GetDriverId() int64 {
	if x != nil {
		return x.DriverId
	}
	return 0
}

type ProductDriverChangedMsg struct {
	state         protoimpl.MessageState `protogen:"open.v1"`
	TenantId      int64                  `protobuf:"varint,1,opt,name=tenantId,proto3" json:"tenantId,omitempty"`
	ProductId     int64                  `protobuf:"varint,2,opt,name=productId,proto3" json:"productId,omitempty"`
	unknownFields protoimpl.UnknownFields
	sizeCache     protoimpl.SizeCache
}

func (x *ProductDriverChangedMsg) Reset() {
	*x = ProductDriverChangedMsg{}
	mi := &file_ng_actor_proto_msgTypes[8]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *ProductDriverChangedMsg) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ProductDriverChangedMsg) ProtoMessage() {}

func (x *ProductDriverChangedMsg) ProtoReflect() protoreflect.Message {
	mi := &file_ng_actor_proto_msgTypes[8]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ProductDriverChangedMsg.ProtoReflect.Descriptor instead.
func (*ProductDriverChangedMsg) Descriptor() ([]byte, []int) {
	return file_ng_actor_proto_rawDescGZIP(), []int{8}
}

func (x *ProductDriverChangedMsg) GetTenantId() int64 {
	if x != nil {
		return x.TenantId
	}
	return 0
}

func (x *ProductDriverChangedMsg) GetProductId() int64 {
	if x != nil {
		return x.ProductId
	}
	return 0
}

var File_ng_actor_proto protoreflect.FileDescriptor

var file_ng_actor_proto_rawDesc = []byte{
	0x0a, 0x0e, 0x6e, 0x67, 0x5f, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x12, 0x08, 0x6e, 0x67, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0f, 0x6e, 0x67, 0x5f, 0x63,
	0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x77, 0x0a, 0x0d, 0x43,
	0x61, 0x63, 0x68, 0x65, 0x45, 0x76, 0x69, 0x63, 0x74, 0x4d, 0x73, 0x67, 0x12, 0x1c, 0x0a, 0x09,
	0x63, 0x61, 0x63, 0x68, 0x65, 0x54, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52,
	0x09, 0x63, 0x61, 0x63, 0x68, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x36, 0x0a, 0x09, 0x65, 0x76,
	0x69, 0x63, 0x74, 0x54, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x18, 0x2e,
	0x6e, 0x67, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x43, 0x61, 0x63, 0x68, 0x65, 0x45, 0x76,
	0x69, 0x63, 0x74, 0x54, 0x79, 0x70, 0x65, 0x52, 0x09, 0x65, 0x76, 0x69, 0x63, 0x74, 0x54, 0x79,
	0x70, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52,
	0x03, 0x6b, 0x65, 0x79, 0x22, 0x67, 0x0a, 0x0d, 0x55, 0x73, 0x65, 0x72, 0x4c, 0x6f, 0x67, 0x6f,
	0x75, 0x74, 0x4d, 0x73, 0x67, 0x12, 0x34, 0x0a, 0x0a, 0x6c, 0x6f, 0x67, 0x6f, 0x75, 0x74, 0x54,
	0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x6e, 0x67, 0x5f, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x4c, 0x6f, 0x67, 0x6f, 0x75, 0x74, 0x54, 0x79, 0x70, 0x65, 0x52,
	0x0a, 0x6c, 0x6f, 0x67, 0x6f, 0x75, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x20, 0x0a, 0x0b, 0x61,
	0x63, 0x63, 0x65, 0x73, 0x73, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x0b, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x22, 0xb6, 0x01,
	0x0a, 0x10, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x4d,
	0x73, 0x67, 0x12, 0x1e, 0x0a, 0x0a, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x54, 0x79, 0x70, 0x65,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x54, 0x79,
	0x70, 0x65, 0x12, 0x3a, 0x0a, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65,
	0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1a, 0x2e, 0x6e, 0x67, 0x5f, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x54, 0x79,
	0x70, 0x65, 0x52, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x14,
	0x0a, 0x05, 0x6e, 0x65, 0x77, 0x49, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x52, 0x05, 0x6e,
	0x65, 0x77, 0x49, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x6f, 0x6c, 0x64, 0x49, 0x64, 0x18, 0x04, 0x20,
	0x01, 0x28, 0x03, 0x52, 0x05, 0x6f, 0x6c, 0x64, 0x49, 0x64, 0x12, 0x1a, 0x0a, 0x08, 0x74, 0x65,
	0x6e, 0x61, 0x6e, 0x74, 0x49, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x03, 0x52, 0x08, 0x74, 0x65,
	0x6e, 0x61, 0x6e, 0x74, 0x49, 0x64, 0x22, 0xae, 0x01, 0x0a, 0x15, 0x44, 0x65, 0x76, 0x69, 0x63,
	0x65, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67,
	0x12, 0x29, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x15,
	0x2e, 0x6e, 0x67, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f,
	0x6e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x2c, 0x0a, 0x05, 0x65,
	0x76, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x16, 0x2e, 0x6e, 0x67, 0x5f,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x45, 0x76, 0x65,
	0x6e, 0x74, 0x52, 0x05, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x3c, 0x0a, 0x0b, 0x73, 0x65, 0x73,
	0x73, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a,
	0x2e, 0x6e, 0x67, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f,
	0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x52, 0x0b, 0x73, 0x65, 0x73, 0x73,
	0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x4a, 0x0a, 0x10, 0x44, 0x65, 0x76, 0x69, 0x63,
	0x65, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x4d, 0x73, 0x67, 0x12, 0x1a, 0x0a, 0x08, 0x74,
	0x65, 0x6e, 0x61, 0x6e, 0x74, 0x49, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x08, 0x74,
	0x65, 0x6e, 0x61, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1a, 0x0a, 0x08, 0x64, 0x65, 0x76, 0x69, 0x63,
	0x65, 0x49, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x08, 0x64, 0x65, 0x76, 0x69, 0x63,
	0x65, 0x49, 0x64, 0x22, 0xa3, 0x01, 0x0a, 0x16, 0x48, 0x6f, 0x75, 0x73, 0x65, 0x6b, 0x65, 0x65,
	0x70, 0x65, 0x72, 0x54, 0x61, 0x73, 0x6b, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1e,
	0x0a, 0x0a, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x54, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x0a, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1a,
	0x0a, 0x08, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x49, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03,
	0x52, 0x08, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x49, 0x64, 0x12, 0x1a, 0x0a, 0x08, 0x74, 0x65,
	0x6e, 0x61, 0x6e, 0x74, 0x49, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x52, 0x08, 0x74, 0x65,
	0x6e, 0x61, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x31, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x04,
	0x20, 0x01, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x6e, 0x67, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
	0x48, 0x6f, 0x75, 0x73, 0x65, 0x6b, 0x65, 0x65, 0x70, 0x65, 0x72, 0x54, 0x61, 0x73, 0x6b, 0x54,
	0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x22, 0x3f, 0x0a, 0x17, 0x48, 0x6f, 0x75,
	0x73, 0x65, 0x6b, 0x65, 0x65, 0x70, 0x65, 0x72, 0x54, 0x61, 0x73, 0x6b, 0x52, 0x65, 0x73, 0x70,
	0x6f, 0x6e, 0x73, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x05, 0x52, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6d, 0x73, 0x67, 0x22, 0x4a, 0x0a, 0x10, 0x44, 0x72,
	0x69, 0x76, 0x65, 0x72, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x4d, 0x73, 0x67, 0x12, 0x1a,
	0x0a, 0x08, 0x74, 0x65, 0x6e, 0x61, 0x6e, 0x74, 0x49, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03,
	0x52, 0x08, 0x74, 0x65, 0x6e, 0x61, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1a, 0x0a, 0x08, 0x64, 0x72,
	0x69, 0x76, 0x65, 0x72, 0x49, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x08, 0x64, 0x72,
	0x69, 0x76, 0x65, 0x72, 0x49, 0x64, 0x22, 0x53, 0x0a, 0x17, 0x50, 0x72, 0x6f, 0x64, 0x75, 0x63,
	0x74, 0x44, 0x72, 0x69, 0x76, 0x65, 0x72, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x4d, 0x73,
	0x67, 0x12, 0x1a, 0x0a, 0x08, 0x74, 0x65, 0x6e, 0x61, 0x6e, 0x74, 0x49, 0x64, 0x18, 0x01, 0x20,
	0x01, 0x28, 0x03, 0x52, 0x08, 0x74, 0x65, 0x6e, 0x61, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1c, 0x0a,
	0x09, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x49, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03,
	0x52, 0x09, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x49, 0x64, 0x2a, 0x4f, 0x0a, 0x10, 0x45,
	0x6e, 0x74, 0x69, 0x74, 0x79, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12,
	0x17, 0x0a, 0x13, 0x45, 0x4e, 0x54, 0x49, 0x54, 0x59, 0x5f, 0x43, 0x48, 0x41, 0x4e, 0x47, 0x45,
	0x5f, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06, 0x43, 0x72, 0x65, 0x61,
	0x74, 0x65, 0x10, 0x01, 0x12, 0x0a, 0x0a, 0x06, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x10, 0x02,
	0x12, 0x0a, 0x0a, 0x06, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x10, 0x03, 0x2a, 0x33, 0x0a, 0x0a,
	0x4c, 0x6f, 0x67, 0x6f, 0x75, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x10, 0x0a, 0x0c, 0x4c, 0x4f,
	0x47, 0x4f, 0x55, 0x54, 0x5f, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04,
	0x53, 0x65, 0x6c, 0x66, 0x10, 0x01, 0x12, 0x09, 0x0a, 0x05, 0x46, 0x6f, 0x72, 0x63, 0x65, 0x10,
	0x02, 0x2a, 0x3c, 0x0a, 0x0e, 0x43, 0x61, 0x63, 0x68, 0x65, 0x45, 0x76, 0x69, 0x63, 0x74, 0x54,
	0x79, 0x70, 0x65, 0x12, 0x15, 0x0a, 0x11, 0x43, 0x41, 0x43, 0x48, 0x45, 0x5f, 0x45, 0x56, 0x49,
	0x43, 0x54, 0x5f, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06, 0x4e, 0x6f,
	0x72, 0x6d, 0x61, 0x6c, 0x10, 0x01, 0x12, 0x07, 0x0a, 0x03, 0x41, 0x6c, 0x6c, 0x10, 0x02, 0x42,
	0x09, 0x5a, 0x07, 0x2e, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x33,
}

var (
	file_ng_actor_proto_rawDescOnce sync.Once
	file_ng_actor_proto_rawDescData = file_ng_actor_proto_rawDesc
)

func file_ng_actor_proto_rawDescGZIP() []byte {
	file_ng_actor_proto_rawDescOnce.Do(func() {
		file_ng_actor_proto_rawDescData = protoimpl.X.CompressGZIP(file_ng_actor_proto_rawDescData)
	})
	return file_ng_actor_proto_rawDescData
}

var file_ng_actor_proto_enumTypes = make([]protoimpl.EnumInfo, 3)
var file_ng_actor_proto_msgTypes = make([]protoimpl.MessageInfo, 9)
var file_ng_actor_proto_goTypes = []any{
	(EntityChangeType)(0),           // 0: ng_proto.EntityChangeType
	(LogoutType)(0),                 // 1: ng_proto.LogoutType
	(CacheEvictType)(0),             // 2: ng_proto.CacheEvictType
	(*CacheEvictMsg)(nil),           // 3: ng_proto.CacheEvictMsg
	(*UserLogoutMsg)(nil),           // 4: ng_proto.UserLogoutMsg
	(*EntityChangedMsg)(nil),        // 5: ng_proto.EntityChangedMsg
	(*DeviceSessionEventMsg)(nil),   // 6: ng_proto.DeviceSessionEventMsg
	(*DeviceDeletedMsg)(nil),        // 7: ng_proto.DeviceDeletedMsg
	(*HousekeeperTaskRequest)(nil),  // 8: ng_proto.HousekeeperTaskRequest
	(*HousekeeperTaskResponse)(nil), // 9: ng_proto.HousekeeperTaskResponse
	(*DriverChangedMsg)(nil),        // 10: ng_proto.DriverChangedMsg
	(*ProductDriverChangedMsg)(nil), // 11: ng_proto.ProductDriverChangedMsg
	(SessionType)(0),                // 12: ng_proto.SessionType
	(SessionEvent)(0),               // 13: ng_proto.SessionEvent
	(*SessionInfoProto)(nil),        // 14: ng_proto.SessionInfoProto
	(HousekeeperTaskType)(0),        // 15: ng_proto.HousekeeperTaskType
}
var file_ng_actor_proto_depIdxs = []int32{
	2,  // 0: ng_proto.CacheEvictMsg.evictType:type_name -> ng_proto.CacheEvictType
	1,  // 1: ng_proto.UserLogoutMsg.logoutType:type_name -> ng_proto.LogoutType
	0,  // 2: ng_proto.EntityChangedMsg.changeType:type_name -> ng_proto.EntityChangeType
	12, // 3: ng_proto.DeviceSessionEventMsg.type:type_name -> ng_proto.SessionType
	13, // 4: ng_proto.DeviceSessionEventMsg.event:type_name -> ng_proto.SessionEvent
	14, // 5: ng_proto.DeviceSessionEventMsg.sessionInfo:type_name -> ng_proto.SessionInfoProto
	15, // 6: ng_proto.HousekeeperTaskRequest.type:type_name -> ng_proto.HousekeeperTaskType
	7,  // [7:7] is the sub-list for method output_type
	7,  // [7:7] is the sub-list for method input_type
	7,  // [7:7] is the sub-list for extension type_name
	7,  // [7:7] is the sub-list for extension extendee
	0,  // [0:7] is the sub-list for field type_name
}

func init() { file_ng_actor_proto_init() }
func file_ng_actor_proto_init() {
	if File_ng_actor_proto != nil {
		return
	}
	file_ng_common_proto_init()
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_ng_actor_proto_rawDesc,
			NumEnums:      3,
			NumMessages:   9,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_ng_actor_proto_goTypes,
		DependencyIndexes: file_ng_actor_proto_depIdxs,
		EnumInfos:         file_ng_actor_proto_enumTypes,
		MessageInfos:      file_ng_actor_proto_msgTypes,
	}.Build()
	File_ng_actor_proto = out.File
	file_ng_actor_proto_rawDesc = nil
	file_ng_actor_proto_goTypes = nil
	file_ng_actor_proto_depIdxs = nil
}
