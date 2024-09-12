resource "aws_api_gateway_rest_api" "pp_service_api" {
  name = "biglnotation_pp_service_api"
}

resource "aws_api_gateway_resource" "pp_service_resource_read" {
  parent_id   = aws_api_gateway_rest_api.pp_service_api.root_resource_id
  rest_api_id = aws_api_gateway_rest_api.pp_service_api.id
  path_part   = "read"
}

resource "aws_api_gateway_method" "pp_service_method_read" {
  authorization = "NONE"
  http_method   = "GET"
  resource_id   = aws_api_gateway_resource.pp_service_resource_read.id
  rest_api_id   = aws_api_gateway_rest_api.pp_service_api.id
}
