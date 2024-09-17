import {
  id = "Z05315841MGPQRJBB5UH"
  to = aws_route53_zone.primary
}

resource "aws_route53_zone" "primary" {
  name = "parkinsonspulse.org"
}

resource "aws_route53_record" "api_alias" {
  name           = "api.${aws_route53_zone.primary.name}"
  zone_id        = aws_route53_zone.primary.zone_id
  type           = "A"

  alias {
    evaluate_target_health = false
    name                   = aws_alb.pp_service.dns_name
    zone_id                = aws_alb.pp_service.zone_id
  }
}