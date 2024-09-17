import {
  id = "Z05315841MGPQRJBB5UH"
  to = aws_route53_zone.primary
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

# resource "aws_acm_certificate_validation" "api_pp_cert_validation" {
#   certificate_arn         = aws_acm_certificate.api_pp_cert.arn
#   validation_record_fqdns = [aws_route53_record.api_alias.fqdn]
#
#   depends_on = [aws_acm_certificate.api_pp_cert]
# }