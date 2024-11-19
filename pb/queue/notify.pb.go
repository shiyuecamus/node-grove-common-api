// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.34.2
// 	protoc        v5.28.3
// source: proto/queue/notify.proto

package queue

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
	return file_proto_queue_notify_proto_enumTypes[0].Descriptor()
}

func (SessionEvent) Type() protoreflect.EnumType {
	return &file_proto_queue_notify_proto_enumTypes[0]
}

func (x SessionEvent) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use SessionEvent.Descriptor instead.
func (SessionEvent) EnumDescriptor() ([]byte, []int) {
	return file_proto_queue_notify_proto_rawDescGZIP(), []int{0}
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
	return file_proto_queue_notify_proto_enumTypes[1].Descriptor()
}

func (SessionType) Type() protoreflect.EnumType {
	return &file_proto_queue_notify_proto_enumTypes[1]
}

func (x SessionType) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use SessionType.Descriptor instead.
func (SessionType) EnumDescriptor() ([]byte, []int) {
	return file_proto_queue_notify_proto_rawDescGZIP(), []int{1}
}

type TransportType int32

const (
	TransportType_TCP   TransportType = 0
	TransportType_MQTT  TransportType = 1
	TransportType_HTTP  TransportType = 2
	TransportType_COAP  TransportType = 3
	TransportType_LWM2M TransportType = 4
	TransportType_SNMP  TransportType = 5
	TransportType_WS    TransportType = 6
)

// Enum value maps for TransportType.
var (
	TransportType_name = map[int32]string{
		0: "TCP",
		1: "MQTT",
		2: "HTTP",
		3: "COAP",
		4: "LWM2M",
		5: "SNMP",
		6: "WS",
	}
	TransportType_value = map[string]int32{
		"TCP":   0,
		"MQTT":  1,
		"HTTP":  2,
		"COAP":  3,
		"LWM2M": 4,
		"SNMP":  5,
		"WS":    6,
	}
)

func (x TransportType) Enum() *TransportType {
	p := new(TransportType)
	*p = x
	return p
}

func (x TransportType) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (TransportType) Descriptor() protoreflect.EnumDescriptor {
	return file_proto_queue_notify_proto_enumTypes[2].Descriptor()
}

func (TransportType) Type() protoreflect.EnumType {
	return &file_proto_queue_notify_proto_enumTypes[2]
}

func (x TransportType) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use TransportType.Descriptor instead.
func (TransportType) EnumDescriptor() ([]byte, []int) {
	return file_proto_queue_notify_proto_rawDescGZIP(), []int{2}
}

type SessionInfoProto struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Transport  TransportType `protobuf:"varint,1,opt,name=transport,proto3,enum=queue.TransportType" json:"transport,omitempty"`
	NodeId     string        `protobuf:"bytes,2,opt,name=nodeId,proto3" json:"nodeId,omitempty"`
	SessionId  string        `protobuf:"bytes,3,opt,name=sessionId,proto3" json:"sessionId,omitempty"`
	TenantId   int64         `protobuf:"varint,4,opt,name=tenantId,proto3" json:"tenantId,omitempty"`
	DeviceId   int64         `protobuf:"varint,5,opt,name=deviceId,proto3" json:"deviceId,omitempty"`
	ProductId  int64         `protobuf:"varint,6,opt,name=productId,proto3" json:"productId,omitempty"`
	DeviceName string        `protobuf:"bytes,7,opt,name=deviceName,proto3" json:"deviceName,omitempty"`
	DeviceType string        `protobuf:"bytes,8,opt,name=deviceType,proto3" json:"deviceType,omitempty"`
}

func (x *SessionInfoProto) Reset() {
	*x = SessionInfoProto{}
	if protoimpl.UnsafeEnabled {
		mi := &file_proto_queue_notify_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *SessionInfoProto) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*SessionInfoProto) ProtoMessage() {}

func (x *SessionInfoProto) ProtoReflect() protoreflect.Message {
	mi := &file_proto_queue_notify_proto_msgTypes[0]
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
	return file_proto_queue_notify_proto_rawDescGZIP(), []int{0}
}

func (x *SessionInfoProto) GetTransport() TransportType {
	if x != nil {
		return x.Transport
	}
	return TransportType_TCP
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

type DeviceSessionEventNotify struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Type        SessionType       `protobuf:"varint,1,opt,name=type,proto3,enum=queue.SessionType" json:"type,omitempty"`
	Event       SessionEvent      `protobuf:"varint,2,opt,name=event,proto3,enum=queue.SessionEvent" json:"event,omitempty"`
	SessionInfo *SessionInfoProto `protobuf:"bytes,3,opt,name=sessionInfo,proto3" json:"sessionInfo,omitempty"`
}

func (x *DeviceSessionEventNotify) Reset() {
	*x = DeviceSessionEventNotify{}
	if protoimpl.UnsafeEnabled {
		mi := &file_proto_queue_notify_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *DeviceSessionEventNotify) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DeviceSessionEventNotify) ProtoMessage() {}

func (x *DeviceSessionEventNotify) ProtoReflect() protoreflect.Message {
	mi := &file_proto_queue_notify_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use DeviceSessionEventNotify.ProtoReflect.Descriptor instead.
func (*DeviceSessionEventNotify) Descriptor() ([]byte, []int) {
	return file_proto_queue_notify_proto_rawDescGZIP(), []int{1}
}

func (x *DeviceSessionEventNotify) GetType() SessionType {
	if x != nil {
		return x.Type
	}
	return SessionType_SESSION_UNSET
}

func (x *DeviceSessionEventNotify) GetEvent() SessionEvent {
	if x != nil {
		return x.Event
	}
	return SessionEvent_SESSION_EVENT_UNSET
}

func (x *DeviceSessionEventNotify) GetSessionInfo() *SessionInfoProto {
	if x != nil {
		return x.SessionInfo
	}
	return nil
}

type EntityCleanupNotify struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	EntityType string `protobuf:"bytes,1,opt,name=entityType,proto3" json:"entityType,omitempty"`
	EntityId   int64  `protobuf:"varint,2,opt,name=entityId,proto3" json:"entityId,omitempty"`
}

func (x *EntityCleanupNotify) Reset() {
	*x = EntityCleanupNotify{}
	if protoimpl.UnsafeEnabled {
		mi := &file_proto_queue_notify_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *EntityCleanupNotify) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*EntityCleanupNotify) ProtoMessage() {}

func (x *EntityCleanupNotify) ProtoReflect() protoreflect.Message {
	mi := &file_proto_queue_notify_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use EntityCleanupNotify.ProtoReflect.Descriptor instead.
func (*EntityCleanupNotify) Descriptor() ([]byte, []int) {
	return file_proto_queue_notify_proto_rawDescGZIP(), []int{2}
}

func (x *EntityCleanupNotify) GetEntityType() string {
	if x != nil {
		return x.EntityType
	}
	return ""
}

func (x *EntityCleanupNotify) GetEntityId() int64 {
	if x != nil {
		return x.EntityId
	}
	return 0
}

var File_proto_queue_notify_proto protoreflect.FileDescriptor

var file_proto_queue_notify_proto_rawDesc = []byte{
	0x0a, 0x18, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x71, 0x75, 0x65, 0x75, 0x65, 0x2f, 0x6e, 0x6f,
	0x74, 0x69, 0x66, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x71, 0x75, 0x65, 0x75,
	0x65, 0x22, 0x92, 0x02, 0x0a, 0x10, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66,
	0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x32, 0x0a, 0x09, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70,
	0x6f, 0x72, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x71, 0x75, 0x65, 0x75,
	0x65, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x54, 0x79, 0x70, 0x65, 0x52,
	0x09, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x6e, 0x6f,
	0x64, 0x65, 0x49, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6e, 0x6f, 0x64, 0x65,
	0x49, 0x64, 0x12, 0x1c, 0x0a, 0x09, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x18,
	0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x49, 0x64,
	0x12, 0x1a, 0x0a, 0x08, 0x74, 0x65, 0x6e, 0x61, 0x6e, 0x74, 0x49, 0x64, 0x18, 0x04, 0x20, 0x01,
	0x28, 0x03, 0x52, 0x08, 0x74, 0x65, 0x6e, 0x61, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1a, 0x0a, 0x08,
	0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x49, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x03, 0x52, 0x08,
	0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x49, 0x64, 0x12, 0x1c, 0x0a, 0x09, 0x70, 0x72, 0x6f, 0x64,
	0x75, 0x63, 0x74, 0x49, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x03, 0x52, 0x09, 0x70, 0x72, 0x6f,
	0x64, 0x75, 0x63, 0x74, 0x49, 0x64, 0x12, 0x1e, 0x0a, 0x0a, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65,
	0x4e, 0x61, 0x6d, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x64, 0x65, 0x76, 0x69,
	0x63, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1e, 0x0a, 0x0a, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65,
	0x54, 0x79, 0x70, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x64, 0x65, 0x76, 0x69,
	0x63, 0x65, 0x54, 0x79, 0x70, 0x65, 0x22, 0xa8, 0x01, 0x0a, 0x18, 0x44, 0x65, 0x76, 0x69, 0x63,
	0x65, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x4e, 0x6f, 0x74,
	0x69, 0x66, 0x79, 0x12, 0x26, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x0e, 0x32, 0x12, 0x2e, 0x71, 0x75, 0x65, 0x75, 0x65, 0x2e, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f,
	0x6e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x29, 0x0a, 0x05, 0x65,
	0x76, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x13, 0x2e, 0x71, 0x75, 0x65,
	0x75, 0x65, 0x2e, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52,
	0x05, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x39, 0x0a, 0x0b, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f,
	0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x71, 0x75,
	0x65, 0x75, 0x65, 0x2e, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x50,
	0x72, 0x6f, 0x74, 0x6f, 0x52, 0x0b, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66,
	0x6f, 0x22, 0x51, 0x0a, 0x13, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x43, 0x6c, 0x65, 0x61, 0x6e,
	0x75, 0x70, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x79, 0x12, 0x1e, 0x0a, 0x0a, 0x65, 0x6e, 0x74, 0x69,
	0x74, 0x79, 0x54, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x65, 0x6e,
	0x74, 0x69, 0x74, 0x79, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x65, 0x6e, 0x74, 0x69,
	0x74, 0x79, 0x49, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x08, 0x65, 0x6e, 0x74, 0x69,
	0x74, 0x79, 0x49, 0x64, 0x2a, 0x3d, 0x0a, 0x0c, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x45,
	0x76, 0x65, 0x6e, 0x74, 0x12, 0x17, 0x0a, 0x13, 0x53, 0x45, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x5f,
	0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x08, 0x0a,
	0x04, 0x4f, 0x50, 0x45, 0x4e, 0x10, 0x01, 0x12, 0x0a, 0x0a, 0x06, 0x43, 0x4c, 0x4f, 0x53, 0x45,
	0x44, 0x10, 0x02, 0x2a, 0x35, 0x0a, 0x0b, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x54, 0x79,
	0x70, 0x65, 0x12, 0x11, 0x0a, 0x0d, 0x53, 0x45, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x5f, 0x55, 0x4e,
	0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x53, 0x59, 0x4e, 0x43, 0x10, 0x01, 0x12,
	0x09, 0x0a, 0x05, 0x41, 0x53, 0x59, 0x4e, 0x43, 0x10, 0x02, 0x2a, 0x53, 0x0a, 0x0d, 0x54, 0x72,
	0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x07, 0x0a, 0x03, 0x54,
	0x43, 0x50, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x4d, 0x51, 0x54, 0x54, 0x10, 0x01, 0x12, 0x08,
	0x0a, 0x04, 0x48, 0x54, 0x54, 0x50, 0x10, 0x02, 0x12, 0x08, 0x0a, 0x04, 0x43, 0x4f, 0x41, 0x50,
	0x10, 0x03, 0x12, 0x09, 0x0a, 0x05, 0x4c, 0x57, 0x4d, 0x32, 0x4d, 0x10, 0x04, 0x12, 0x08, 0x0a,
	0x04, 0x53, 0x4e, 0x4d, 0x50, 0x10, 0x05, 0x12, 0x06, 0x0a, 0x02, 0x57, 0x53, 0x10, 0x06, 0x42,
	0x0e, 0x5a, 0x0c, 0x2e, 0x2f, 0x2e, 0x2f, 0x70, 0x62, 0x2f, 0x71, 0x75, 0x65, 0x75, 0x65, 0x62,
	0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_proto_queue_notify_proto_rawDescOnce sync.Once
	file_proto_queue_notify_proto_rawDescData = file_proto_queue_notify_proto_rawDesc
)

func file_proto_queue_notify_proto_rawDescGZIP() []byte {
	file_proto_queue_notify_proto_rawDescOnce.Do(func() {
		file_proto_queue_notify_proto_rawDescData = protoimpl.X.CompressGZIP(file_proto_queue_notify_proto_rawDescData)
	})
	return file_proto_queue_notify_proto_rawDescData
}

var file_proto_queue_notify_proto_enumTypes = make([]protoimpl.EnumInfo, 3)
var file_proto_queue_notify_proto_msgTypes = make([]protoimpl.MessageInfo, 3)
var file_proto_queue_notify_proto_goTypes = []any{
	(SessionEvent)(0),                // 0: queue.SessionEvent
	(SessionType)(0),                 // 1: queue.SessionType
	(TransportType)(0),               // 2: queue.TransportType
	(*SessionInfoProto)(nil),         // 3: queue.SessionInfoProto
	(*DeviceSessionEventNotify)(nil), // 4: queue.DeviceSessionEventNotify
	(*EntityCleanupNotify)(nil),      // 5: queue.EntityCleanupNotify
}
var file_proto_queue_notify_proto_depIdxs = []int32{
	2, // 0: queue.SessionInfoProto.transport:type_name -> queue.TransportType
	1, // 1: queue.DeviceSessionEventNotify.type:type_name -> queue.SessionType
	0, // 2: queue.DeviceSessionEventNotify.event:type_name -> queue.SessionEvent
	3, // 3: queue.DeviceSessionEventNotify.sessionInfo:type_name -> queue.SessionInfoProto
	4, // [4:4] is the sub-list for method output_type
	4, // [4:4] is the sub-list for method input_type
	4, // [4:4] is the sub-list for extension type_name
	4, // [4:4] is the sub-list for extension extendee
	0, // [0:4] is the sub-list for field type_name
}

func init() { file_proto_queue_notify_proto_init() }
func file_proto_queue_notify_proto_init() {
	if File_proto_queue_notify_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_proto_queue_notify_proto_msgTypes[0].Exporter = func(v any, i int) any {
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
		file_proto_queue_notify_proto_msgTypes[1].Exporter = func(v any, i int) any {
			switch v := v.(*DeviceSessionEventNotify); i {
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
		file_proto_queue_notify_proto_msgTypes[2].Exporter = func(v any, i int) any {
			switch v := v.(*EntityCleanupNotify); i {
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
			RawDescriptor: file_proto_queue_notify_proto_rawDesc,
			NumEnums:      3,
			NumMessages:   3,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_proto_queue_notify_proto_goTypes,
		DependencyIndexes: file_proto_queue_notify_proto_depIdxs,
		EnumInfos:         file_proto_queue_notify_proto_enumTypes,
		MessageInfos:      file_proto_queue_notify_proto_msgTypes,
	}.Build()
	File_proto_queue_notify_proto = out.File
	file_proto_queue_notify_proto_rawDesc = nil
	file_proto_queue_notify_proto_goTypes = nil
	file_proto_queue_notify_proto_depIdxs = nil
}
