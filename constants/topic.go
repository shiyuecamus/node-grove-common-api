package constants

var (
	ProvisionRequestTopic  = "/provision/req"
	ProvisionResponseTopic = "/provision/res"

	baseDeviceApiTopic = "/v1/d"
	DeviceInfoTopic    = baseDeviceApiTopic + "/i"
	SelfAttributeTopic = baseDeviceApiTopic + "/a"
	SelfTelemetryTopic = baseDeviceApiTopic + "/t"
	SelfRpcTopic       = baseDeviceApiTopic + "/r"

	baseGatewayApiTopic = "/v1/g"
)
