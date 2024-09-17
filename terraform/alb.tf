resource "aws_alb" "pp_service" {
  name = "biglnotation-pp-service-alb"
  internal = false
  load_balancer_type = "application"

  subnets = [aws_subnet.sn1_private.id, aws_subnet.sn2_public.id]
  security_groups = [aws_security_group.pp_service_alb_sg.id]

  enable_deletion_protection = false
  enable_http2 = true
  enable_cross_zone_load_balancing = true
}

resource "aws_lb_target_group" "pp_service_tg" {
  name        = "test-tg"
  port        = 4444
  protocol    = "HTTP"
  target_type = "ip"
  vpc_id      = aws_vpc.vpc.id
}

resource "aws_lb_listener" "listener" {
  load_balancer_arn         = aws_alb.pp_service.arn
  port                      = "80"
  protocol                  = "HTTP"

  default_action {
    type                    = "forward"
    target_group_arn        = aws_lb_target_group.pp_service_tg.arn
  }
}