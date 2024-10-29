resource "aws_alb" "pp_service" {
  name               = "biglnotation-pp-service-alb"
  internal           = false
  load_balancer_type = "application"

  subnets         = [aws_subnet.sn1_public.id, aws_subnet.sn2_public.id]
  security_groups = [aws_security_group.pp_service_alb_sg.id]

  enable_deletion_protection       = false
  enable_http2                     = true
  enable_cross_zone_load_balancing = true
}

resource "aws_lb_target_group" "pp_service_tg" {
  name        = "biglnotation-pp-service-tg"
  port        = 4444
  protocol    = "HTTP"
  target_type = "ip"
  vpc_id      = aws_vpc.vpc.id
}

resource "aws_lb_listener" "https" {
  load_balancer_arn = aws_alb.pp_service.arn
  port              = "443"
  protocol          = "HTTPS"

  ssl_policy      = "ELBSecurityPolicy-2016-08"
  certificate_arn = aws_acm_certificate.api_pp_cert.arn

  default_action {
    type = "fixed-response"

    fixed_response {
      content_type = "text/html"
      message_body = "Direct access is denied"
      status_code  = "401"
    }
  }
}

resource "aws_lb_listener" "http" {
  load_balancer_arn = aws_alb.pp_service.arn
  port              = "80"
  protocol          = "HTTP"

  default_action {
    type = "redirect"

    redirect {
      port        = "443"
      protocol    = "HTTPS"
      status_code = "HTTP_301"
    }
  }
}