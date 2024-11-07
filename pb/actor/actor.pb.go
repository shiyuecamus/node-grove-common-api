// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.34.2
// 	protoc        v5.27.0
// source: proto/actor/actor.proto

package actor

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
	return file_proto_actor_actor_proto_enumTypes[0].Descriptor()
}

func (EntityChangeType) Type() protoreflect.EnumType {
	return &file_proto_actor_actor_proto_enumTypes[0]
}

func (x EntityChangeType) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use EntityChangeType.Descriptor instead.
func (EntityChangeType) EnumDescriptor() ([]byte, []int) {
	return file_proto_actor_actor_proto_rawDescGZIP(), []int{0}
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
	return file_proto_actor_actor_proto_enumTypes[1].Descriptor()
}

func (LogoutType) Type() protoreflect.EnumType {
	return &file_proto_actor_actor_proto_enumTypes[1]
}

func (x LogoutType) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use LogoutType.Descriptor instead.
func (LogoutType) EnumDescriptor() ([]byte, []int) {
	return file_proto_actor_actor_proto_rawDescGZIP(), []int{1}
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
	return file_proto_actor_actor_proto_enumTypes[2].Descriptor()
}

func (CacheEvictType) Type() protoreflect.EnumType {
	return &file_proto_actor_actor_proto_enumTypes[2]
}

func (x CacheEvictType) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use CacheEvictType.Descriptor instead.
func (CacheEvictType) EnumDescriptor() ([]byte, []int) {
	return file_proto_actor_actor_proto_rawDescGZIP(), []int{2}
}

type SessionEvent int32

const (
	SessionEvent_SESSION_EVENT_UNSET SessionEvent = 0
	SessionEvent_OPEN                SessionEvent = 1
	SessionEvent_CLOSED              SessionEvent = 2
)

// Enum value maps for SessionEvent.
var (
	SessionEvent_name = map[int32]string{
		0: "SESSION_EVENT_UNSET",
		1: "OPEN",
		2: "CLOSED",
	}
	SessionEvent_value = map[string]int32{
		"SESSION_EVENT_UNSET": 0,
		"OPEN":                1,
		"CLOSED":              2,
	}
)

func (x SessionEvent) Enum() *SessionEvent {
	p := new(SessionEvent)
	*p = x
	return p
}

func (x SessionEvent) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (SessionEvent) Descriptor() protoreflect.EnumDescriptor {
	return file_proto_actor_actor_proto_enumTypes[3].Descriptor()
}

func (SessionEvent) Type() protoreflect.EnumType {
	return &file_proto_actor_actor_proto_enumTypes[3]
}

func (x SessionEvent) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use SessionEvent.Descriptor instead.
func (SessionEvent) EnumDescriptor() ([]byte, []int) {
	return file_proto_actor_actor_proto_rawDescGZIP(), []int{3}
}

type SessionType int32

const (
	SessionType_SESSION_UNSET SessionType = 0
	SessionType_SYNC          SessionType = 1
	SessionType_ASYNC         SessionType = 2
)

// Enum value maps for SessionType.
var (
	SessionType_name = map[int32]string{
		0: "SESSION_UNSET",
		1: "SYNC",
		2: "ASYNC",
	}
	SessionType_value = map[string]int32{
		"SESSION_UNSET": 0,
		"SYNC":          1,
		"ASYNC":         2,
	}
)

func (x SessionType) Enum() *SessionType {
	p := new(SessionType)
	*p = x
	return p
}

func (x SessionType) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (SessionType) Descriptor() protoreflect.EnumDescriptor {
	return file_proto_actor_actor_proto_enumTypes[4].Descriptor()
}

func (SessionType) Type() protoreflect.EnumType {
	return &file_proto_actor_actor_proto_enumTypes[4]
}

func (x SessionType) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use SessionType.Descriptor instead.
func (SessionType) EnumDescriptor() ([]byte, []int) {
	return file_proto_actor_actor_proto_rawDescGZIP(), []int{4}
}

type CacheEvictMsg struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	CacheType int32          `protobuf:"varint,1,opt,name=cacheType,proto3" json:"cacheType,omitempty"`
	EvictType CacheEvictType `protobuf:"varint,2,opt,name=evictType,proto3,enum=actor.CacheEvictType" json:"evictType,omitempty"`
	Key       string         `protobuf:"bytes,3,opt,name=key,proto3" json:"key,omitempty"`
}

func (x *CacheEvictMsg) Reset() {
	*x = CacheEvictMsg{}
	if protoimpl.UnsafeEnabled {
		mi := &file_proto_actor_actor_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CacheEvictMsg) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CacheEvictMsg) ProtoMessage() {}

func (x *CacheEvictMsg) ProtoReflect() protoreflect.Message {
	mi := &file_proto_actor_actor_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
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
	return file_proto_actor_actor_proto_rawDescGZIP(), []int{0}
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
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	LogoutType  LogoutType `protobuf:"varint,1,opt,name=logoutType,proto3,enum=actor.LogoutType" json:"logoutType,omitempty"`
	AccessToken string     `protobuf:"bytes,2,opt,name=accessToken,proto3" json:"accessToken,omitempty"`
}

func (x *UserLogoutMsg) Reset() {
	*x = UserLogoutMsg{}
	if protoimpl.UnsafeEnabled {
		mi := &file_proto_actor_actor_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *UserLogoutMsg) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UserLogoutMsg) ProtoMessage() {}

func (x *UserLogoutMsg) ProtoReflect() protoreflect.Message {
	mi := &file_proto_actor_actor_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
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
	return file_proto_actor_actor_proto_rawDescGZIP(), []int{1}
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
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	EntityType string           `protobuf:"bytes,1,opt,name=entityType,proto3" json:"entityType,omitempty"`
	ChangeType EntityChangeType `protobuf:"varint,2,opt,name=changeType,proto3,enum=actor.EntityChangeType" json:"changeType,omitempty"`
	NewId      int32            `protobuf:"varint,3,opt,name=newId,proto3" json:"newId,omitempty"`
	OldId      int32            `protobuf:"varint,4,opt,name=oldId,proto3" json:"oldId,omitempty"`
}

func (x *EntityChangedMsg) Reset() {
	*x = EntityChangedMsg{}
	if protoimpl.UnsafeEnabled {
		mi := &file_proto_actor_actor_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *EntityChangedMsg) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*EntityChangedMsg) ProtoMessage() {}

func (x *EntityChangedMsg) ProtoReflect() protoreflect.Message {
	mi := &file_proto_actor_actor_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
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
	return file_proto_actor_actor_proto_rawDescGZIP(), []int{2}
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

func (x *EntityChangedMsg) GetNewId() int32 {
	if x != nil {
		return x.NewId
	}
	return 0
}

func (x *EntityChangedMsg) GetOldId() int32 {
	if x != nil {
		return x.OldId
	}
	return 0
}

type SessionInfoProto struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	NodeId     string `protobuf:"bytes,1,opt,name=nodeId,proto3" json:"nodeId,omitempty"`
	SessionId  string `protobuf:"bytes,2,opt,name=sessionId,proto3" json:"sessionId,omitempty"`
	TenantId   int64  `protobuf:"varint,3,opt,name=tenantId,proto3" json:"tenantId,omitempty"`
	DeviceId   int64  `protobuf:"varint,4,opt,name=deviceId,proto3" json:"deviceId,omitempty"`
	ProductId  int64  `protobuf:"varint,5,opt,name=productId,proto3" json:"productId,omitempty"`
	DeviceName string `protobuf:"bytes,6,opt,name=deviceName,proto3" json:"deviceName,omitempty"`
	DeviceType string `protobuf:"bytes,7,opt,name=deviceType,proto3" json:"deviceType,omitempty"`
}

func (x *SessionInfoProto) Reset() {
	*x = SessionInfoProto{}
	if protoimpl.UnsafeEnabled {
		mi := &file_proto_actor_actor_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *SessionInfoProto) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*SessionInfoProto) ProtoMessage() {}

func (x *SessionInfoProto) ProtoReflect() protoreflect.Message {
	mi := &file_proto_actor_actor_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use SessionInfoProto.ProtoReflect.Descriptor instead.
func (*SessionInfoProto) Descriptor() ([]byte, []int) {
	return file_proto_actor_actor_proto_rawDescGZIP(), []int{3}
}

func (x *SessionInfoProto) GetNodeId() string {
	if x != nil {
		return x.NodeId
	}
	return ""
}

func (x *SessionInfoProto) GetSessionId() string {
	if x != nil {
		return x.SessionId
	}
	return ""
}

func (x *SessionInfoProto) GetTenantId() int64 {
	if x != nil {
		return x.TenantId
	}
	return 0
}

func (x *SessionInfoProto) GetDeviceId() int64 {
	if x != nil {
		return x.DeviceId
	}
	return 0
}

func (x *SessionInfoProto) GetProductId() int64 {
	if x != nil {
		return x.ProductId
	}
	return 0
}

func (x *SessionInfoProto) GetDeviceName() string {
	if x != nil {
		return x.DeviceName
	}
	return ""
}

func (x *SessionInfoProto) GetDeviceType() string {
	if x != nil {
		return x.DeviceType
	}
	return ""
}

type DeviceSessionEventMsg struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Type        SessionType       `protobuf:"varint,1,opt,name=type,proto3,enum=actor.SessionType" json:"type,omitempty"`
	Event       SessionEvent      `protobuf:"varint,2,opt,name=event,proto3,enum=actor.SessionEvent" json:"event,omitempty"`
	SessionInfo *SessionInfoProto `protobuf:"bytes,3,opt,name=sessionInfo,proto3" json:"sessionInfo,omitempty"`
}

func (x *DeviceSessionEventMsg) Reset() {
	*x = DeviceSessionEventMsg{}
	if protoimpl.UnsafeEnabled {
		mi := &file_proto_actor_actor_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *DeviceSessionEventMsg) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DeviceSessionEventMsg) ProtoMessage() {}

func (x *DeviceSessionEventMsg) ProtoReflect() protoreflect.Message {
	mi := &file_proto_actor_actor_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
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
	return file_proto_actor_actor_proto_rawDescGZIP(), []int{4}
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

var File_proto_actor_actor_proto protoreflect.FileDescriptor

var file_proto_actor_actor_proto_rawDesc = []byte{
	0x0a, 0x17, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x2f, 0x61, 0x63,
	0x74, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x61, 0x63, 0x74, 0x6f, 0x72,
	0x22, 0x74, 0x0a, 0x0d, 0x43, 0x61, 0x63, 0x68, 0x65, 0x45, 0x76, 0x69, 0x63, 0x74, 0x4d, 0x73,
	0x67, 0x12, 0x1c, 0x0a, 0x09, 0x63, 0x61, 0x63, 0x68, 0x65, 0x54, 0x79, 0x70, 0x65, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x05, 0x52, 0x09, 0x63, 0x61, 0x63, 0x68, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12,
	0x33, 0x0a, 0x09, 0x65, 0x76, 0x69, 0x63, 0x74, 0x54, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01,
	0x28, 0x0e, 0x32, 0x15, 0x2e, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x2e, 0x43, 0x61, 0x63, 0x68, 0x65,
	0x45, 0x76, 0x69, 0x63, 0x74, 0x54, 0x79, 0x70, 0x65, 0x52, 0x09, 0x65, 0x76, 0x69, 0x63, 0x74,
	0x54, 0x79, 0x70, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x22, 0x64, 0x0a, 0x0d, 0x55, 0x73, 0x65, 0x72, 0x4c, 0x6f,
	0x67, 0x6f, 0x75, 0x74, 0x4d, 0x73, 0x67, 0x12, 0x31, 0x0a, 0x0a, 0x6c, 0x6f, 0x67, 0x6f, 0x75,
	0x74, 0x54, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x61, 0x63,
	0x74, 0x6f, 0x72, 0x2e, 0x4c, 0x6f, 0x67, 0x6f, 0x75, 0x74, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0a,
	0x6c, 0x6f, 0x67, 0x6f, 0x75, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x20, 0x0a, 0x0b, 0x61, 0x63,
	0x63, 0x65, 0x73, 0x73, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
	0x0b, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x22, 0x97, 0x01, 0x0a,
	0x10, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x4d, 0x73,
	0x67, 0x12, 0x1e, 0x0a, 0x0a, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x54, 0x79, 0x70, 0x65, 0x18,
	0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x54, 0x79, 0x70,
	0x65, 0x12, 0x37, 0x0a, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x17, 0x2e, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x2e, 0x45, 0x6e,
	0x74, 0x69, 0x74, 0x79, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0a,
	0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x6e, 0x65,
	0x77, 0x49, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x05, 0x6e, 0x65, 0x77, 0x49, 0x64,
	0x12, 0x14, 0x0a, 0x05, 0x6f, 0x6c, 0x64, 0x49, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x52,
	0x05, 0x6f, 0x6c, 0x64, 0x49, 0x64, 0x22, 0xde, 0x01, 0x0a, 0x10, 0x53, 0x65, 0x73, 0x73, 0x69,
	0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x16, 0x0a, 0x06, 0x6e,
	0x6f, 0x64, 0x65, 0x49, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6e, 0x6f, 0x64,
	0x65, 0x49, 0x64, 0x12, 0x1c, 0x0a, 0x09, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x49, 0x64,
	0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x49,
	0x64, 0x12, 0x1a, 0x0a, 0x08, 0x74, 0x65, 0x6e, 0x61, 0x6e, 0x74, 0x49, 0x64, 0x18, 0x03, 0x20,
	0x01, 0x28, 0x03, 0x52, 0x08, 0x74, 0x65, 0x6e, 0x61, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1a, 0x0a,
	0x08, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x49, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x52,
	0x08, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x49, 0x64, 0x12, 0x1c, 0x0a, 0x09, 0x70, 0x72, 0x6f,
	0x64, 0x75, 0x63, 0x74, 0x49, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x03, 0x52, 0x09, 0x70, 0x72,
	0x6f, 0x64, 0x75, 0x63, 0x74, 0x49, 0x64, 0x12, 0x1e, 0x0a, 0x0a, 0x64, 0x65, 0x76, 0x69, 0x63,
	0x65, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x64, 0x65, 0x76,
	0x69, 0x63, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1e, 0x0a, 0x0a, 0x64, 0x65, 0x76, 0x69, 0x63,
	0x65, 0x54, 0x79, 0x70, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x64, 0x65, 0x76,
	0x69, 0x63, 0x65, 0x54, 0x79, 0x70, 0x65, 0x22, 0xa5, 0x01, 0x0a, 0x15, 0x44, 0x65, 0x76, 0x69,
	0x63, 0x65, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x4d, 0x73,
	0x67, 0x12, 0x26, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32,
	0x12, 0x2e, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x2e, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x54,
	0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x29, 0x0a, 0x05, 0x65, 0x76, 0x65,
	0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x13, 0x2e, 0x61, 0x63, 0x74, 0x6f, 0x72,
	0x2e, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x05, 0x65,
	0x76, 0x65, 0x6e, 0x74, 0x12, 0x39, 0x0a, 0x0b, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x49,
	0x6e, 0x66, 0x6f, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x61, 0x63, 0x74, 0x6f,
	0x72, 0x2e, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f,
	0x74, 0x6f, 0x52, 0x0b, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x2a,
	0x4f, 0x0a, 0x10, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x54,
	0x79, 0x70, 0x65, 0x12, 0x17, 0x0a, 0x13, 0x45, 0x4e, 0x54, 0x49, 0x54, 0x59, 0x5f, 0x43, 0x48,
	0x41, 0x4e, 0x47, 0x45, 0x5f, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06,
	0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x10, 0x01, 0x12, 0x0a, 0x0a, 0x06, 0x55, 0x70, 0x64, 0x61,
	0x74, 0x65, 0x10, 0x02, 0x12, 0x0a, 0x0a, 0x06, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x10, 0x03,
	0x2a, 0x33, 0x0a, 0x0a, 0x4c, 0x6f, 0x67, 0x6f, 0x75, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x10,
	0x0a, 0x0c, 0x4c, 0x4f, 0x47, 0x4f, 0x55, 0x54, 0x5f, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00,
	0x12, 0x08, 0x0a, 0x04, 0x53, 0x65, 0x6c, 0x66, 0x10, 0x01, 0x12, 0x09, 0x0a, 0x05, 0x46, 0x6f,
	0x72, 0x63, 0x65, 0x10, 0x02, 0x2a, 0x3c, 0x0a, 0x0e, 0x43, 0x61, 0x63, 0x68, 0x65, 0x45, 0x76,
	0x69, 0x63, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x15, 0x0a, 0x11, 0x43, 0x41, 0x43, 0x48, 0x45,
	0x5f, 0x45, 0x56, 0x49, 0x43, 0x54, 0x5f, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0a,
	0x0a, 0x06, 0x4e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x10, 0x01, 0x12, 0x07, 0x0a, 0x03, 0x41, 0x6c,
	0x6c, 0x10, 0x02, 0x2a, 0x3d, 0x0a, 0x0c, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x45, 0x76,
	0x65, 0x6e, 0x74, 0x12, 0x17, 0x0a, 0x13, 0x53, 0x45, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x5f, 0x45,
	0x56, 0x45, 0x4e, 0x54, 0x5f, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04,
	0x4f, 0x50, 0x45, 0x4e, 0x10, 0x01, 0x12, 0x0a, 0x0a, 0x06, 0x43, 0x4c, 0x4f, 0x53, 0x45, 0x44,
	0x10, 0x02, 0x2a, 0x35, 0x0a, 0x0b, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x54, 0x79, 0x70,
	0x65, 0x12, 0x11, 0x0a, 0x0d, 0x53, 0x45, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x5f, 0x55, 0x4e, 0x53,
	0x45, 0x54, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x53, 0x59, 0x4e, 0x43, 0x10, 0x01, 0x12, 0x09,
	0x0a, 0x05, 0x41, 0x53, 0x59, 0x4e, 0x43, 0x10, 0x02, 0x42, 0x0e, 0x5a, 0x0c, 0x2e, 0x2f, 0x2e,
	0x2f, 0x70, 0x62, 0x2f, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x33,
}

var (
	file_proto_actor_actor_proto_rawDescOnce sync.Once
	file_proto_actor_actor_proto_rawDescData = file_proto_actor_actor_proto_rawDesc
)

func file_proto_actor_actor_proto_rawDescGZIP() []byte {
	file_proto_actor_actor_proto_rawDescOnce.Do(func() {
		file_proto_actor_actor_proto_rawDescData = protoimpl.X.CompressGZIP(file_proto_actor_actor_proto_rawDescData)
	})
	return file_proto_actor_actor_proto_rawDescData
}

var file_proto_actor_actor_proto_enumTypes = make([]protoimpl.EnumInfo, 5)
var file_proto_actor_actor_proto_msgTypes = make([]protoimpl.MessageInfo, 5)
var file_proto_actor_actor_proto_goTypes = []any{
	(EntityChangeType)(0),         // 0: actor.EntityChangeType
	(LogoutType)(0),               // 1: actor.LogoutType
	(CacheEvictType)(0),           // 2: actor.CacheEvictType
	(SessionEvent)(0),             // 3: actor.SessionEvent
	(SessionType)(0),              // 4: actor.SessionType
	(*CacheEvictMsg)(nil),         // 5: actor.CacheEvictMsg
	(*UserLogoutMsg)(nil),         // 6: actor.UserLogoutMsg
	(*EntityChangedMsg)(nil),      // 7: actor.EntityChangedMsg
	(*SessionInfoProto)(nil),      // 8: actor.SessionInfoProto
	(*DeviceSessionEventMsg)(nil), // 9: actor.DeviceSessionEventMsg
}
var file_proto_actor_actor_proto_depIdxs = []int32{
	2, // 0: actor.CacheEvictMsg.evictType:type_name -> actor.CacheEvictType
	1, // 1: actor.UserLogoutMsg.logoutType:type_name -> actor.LogoutType
	0, // 2: actor.EntityChangedMsg.changeType:type_name -> actor.EntityChangeType
	4, // 3: actor.DeviceSessionEventMsg.type:type_name -> actor.SessionType
	3, // 4: actor.DeviceSessionEventMsg.event:type_name -> actor.SessionEvent
	8, // 5: actor.DeviceSessionEventMsg.sessionInfo:type_name -> actor.SessionInfoProto
	6, // [6:6] is the sub-list for method output_type
	6, // [6:6] is the sub-list for method input_type
	6, // [6:6] is the sub-list for extension type_name
	6, // [6:6] is the sub-list for extension extendee
	0, // [0:6] is the sub-list for field type_name
}

func init() { file_proto_actor_actor_proto_init() }
func file_proto_actor_actor_proto_init() {
	if File_proto_actor_actor_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_proto_actor_actor_proto_msgTypes[0].Exporter = func(v any, i int) any {
			switch v := v.(*CacheEvictMsg); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_proto_actor_actor_proto_msgTypes[1].Exporter = func(v any, i int) any {
			switch v := v.(*UserLogoutMsg); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_proto_actor_actor_proto_msgTypes[2].Exporter = func(v any, i int) any {
			switch v := v.(*EntityChangedMsg); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_proto_actor_actor_proto_msgTypes[3].Exporter = func(v any, i int) any {
			switch v := v.(*SessionInfoProto); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_proto_actor_actor_proto_msgTypes[4].Exporter = func(v any, i int) any {
			switch v := v.(*DeviceSessionEventMsg); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_proto_actor_actor_proto_rawDesc,
			NumEnums:      5,
			NumMessages:   5,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_proto_actor_actor_proto_goTypes,
		DependencyIndexes: file_proto_actor_actor_proto_depIdxs,
		EnumInfos:         file_proto_actor_actor_proto_enumTypes,
		MessageInfos:      file_proto_actor_actor_proto_msgTypes,
	}.Build()
	File_proto_actor_actor_proto = out.File
	file_proto_actor_actor_proto_rawDesc = nil
	file_proto_actor_actor_proto_goTypes = nil
	file_proto_actor_actor_proto_depIdxs = nil
}
