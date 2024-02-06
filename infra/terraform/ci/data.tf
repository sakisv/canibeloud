data "pass_password" "ci_ssh_private_key" { path = "credentials/ci/ssh_private_key" }
data "pass_password" "ci_ssh_public_key" { path = "credentials/ci/ssh_public_key" }
data "pass_password" "ci_email_address_sender" { path = "credentials/ci/email_address_sender" }
data "pass_password" "ci_email_address_recipient" { path = "credentials/ci/email_address_recipient" }
data "pass_password" "dockerhub_api_password" { path = "credentials/dockerhub/api_password" }
data "pass_password" "dockerhub_api_username" { path = "credentials/dockerhub/api_username" }
data "pass_password" "fastmail_username" { path = "credentials/fastmail/username" }
data "pass_password" "fastmail_password" { path = "credentials/fastmail/password" }
