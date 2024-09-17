resource "aws_ecs_cluster" "pp_service_cluster" {
  name = "biglnotation_pp_service_cluster"
}

resource "aws_cloudwatch_log_group" "pp_service_lg" {
  name = "biglnotation/pp/pp_service_cluster"

  tags = {
    Application = "pp_service"
  }
}

resource "aws_ecs_service" "pp_service_service" {
  name                   = "biglnotation_pp_service_service"
  cluster                = aws_ecs_cluster.pp_service_cluster.arn
  launch_type            = "FARGATE"
  enable_execute_command = true

  deployment_maximum_percent         = 200
  deployment_minimum_healthy_percent = 100
  desired_count                      = 1
  task_definition                    = aws_ecs_task_definition.pp_service_td.arn

  network_configuration {
    assign_public_ip = true
    security_groups  = [aws_security_group.public_sg.id, aws_security_group.user_db_connection_sg.id]
    subnets = [aws_subnet.sn1_public.id, aws_subnet.sn2_public.id,
    aws_subnet.sn1_private.id, aws_subnet.sn2_private.id]
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.pp_service_tg.arn
    container_name   = "app"
    container_port   = 4444
  }

  #   depends_on = [aws_lb_listener.https]
}

resource "aws_ecs_task_definition" "pp_service_td" {
  family                   = "pp_service"
  requires_compatibilities = ["FARGATE"]

  cpu                = "256"
  memory             = "512"
  network_mode       = "awsvpc"
  task_role_arn      = "arn:aws:iam::025066274148:role/ecsTaskRole"
  execution_role_arn = "arn:aws:iam::025066274148:role/ecsTaskRole"

  container_definitions = jsonencode([
    {
      name      = "app"
      image     = "${aws_ecr_repository.pp_service_repo.repository_url}:latest"
      cpu       = 256
      memory    = 512
      essential = true
      portMappings = [
        {
          containerPort = 4444
          hostPort      = 4444
        },
        {
          containerPort = 27017
          hostPort      = 27017
        }
      ]
      environment = [
        {
          name  = "DATABASE_URL",
          value = "mongodb://${aws_docdb_cluster.user_db.master_username}:${aws_docdb_cluster.user_db.master_password}@biglnotation-user-db.cluster-cf8y42qogmho.ap-southeast-2.docdb.amazonaws.com:27017"
        },
      ]
      healthCheck = {
        command = ["CMD-SHELL", "curl -f http://localhost:2222/health || exit 1"],
        retries = 10,
        timeout : 5
        interval : 10
      }
      logConfiguration = {
        logDriver = "awslogs"
        "options" : {
          "awslogs-group" : aws_cloudwatch_log_group.pp_service_lg.name,
          "awslogs-region" : "ap-southeast-2",
          "awslogs-stream-prefix" : "pp-service-task"
        }
      }
    }
  ])
}