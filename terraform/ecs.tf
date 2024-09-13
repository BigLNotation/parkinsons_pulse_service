resource "aws_ecs_cluster" "pp_service_cluster" {
  name = "biglnotation_pp_service_cluster"
}

resource "aws_ecs_service" "pp_service_service" {
  name = "biglnotation_pp_service_service"
  cluster = aws_ecs_cluster.pp_service_cluster.arn
  launch_type = "FARGATE"
  enable_execute_command = true

  deployment_maximum_percent = 200
  deployment_minimum_healthy_percent = 100
  desired_count = 1
  task_definition = aws_ecs_task_definition.pp_service_td.arn

  network_configuration {
    assign_public_ip = true
#     security_groups = [aws_security_group.sg.id]
    subnets = [aws_subnet.sn1_public.id, aws_subnet.sn2_public.id]
  }
}

resource "aws_ecs_task_definition" "pp_service_td" {
  container_definitions = jsonencode([
    {
      name         = "app"
      image        = "hashicorp/http-echo:latest"
      cpu          = 256
      memory       = 512
      essential    = true
      portMappings = [
        {
          containerPort = 5678
          hostPort      = 5678
        }
      ]
    }
  ])
  family                   = "pp_service"
  requires_compatibilities = ["FARGATE"]

  cpu                = "256"
  memory             = "512"
  network_mode       = "awsvpc"
  task_role_arn      = "arn:aws:iam::278336116187:role/ecsTaskExecutionRole"
  execution_role_arn = "arn:aws:iam::278336116187:role/ecsTaskExecutionRole"
}