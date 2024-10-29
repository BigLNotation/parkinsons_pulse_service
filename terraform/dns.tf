import {
  id = "Z05315841MGPQRJBB5UH"
  to = aws_route53_zone.primary
}

import {
  id = "arn:aws:acm:ap-southeast-2:025066274148:certificate/c823ddea-41fb-42e7-b679-d17a096c204b"
  to = aws_acm_certificate.api_pp_cert
}

resource "aws_route53_zone" "primary" {
  name = "parkinsonspulse.org"
}

resource "aws_route53_record" "api_alias" {
  name    = "api.${aws_route53_zone.primary.name}"
  zone_id = aws_route53_zone.primary.zone_id
  type    = "A"

  alias {
    evaluate_target_health = false
    name                   = aws_alb.pp_service.dns_name
    zone_id                = aws_alb.pp_service.zone_id
  }
}

resource "aws_acm_certificate" "api_pp_cert" {
  domain_name       = aws_route53_record.api_alias.fqdn
  validation_method = "DNS"

  lifecycle {
    create_before_destroy = true
  }
}