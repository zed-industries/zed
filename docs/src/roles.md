---
title: Roles - Zed
description: Understand Zed's organization roles and what each role can access, manage, and configure.
---

# Roles

Every member of a Zed organization is assigned a role that determines what they
can access and configure.

## Role Types {#roles}

Every member of an organization is assigned one of four roles:

| Capability                                                      | Owner | Admin | Billing Manager | Member |
| --------------------------------------------------------------- | ----- | ----- | --------------- | ------ |
| Use hosted AI and Edit Predictions through Business             | Yes   | Yes   | No              | Yes    |
| View organization members                                       | Yes   | Yes   | Yes             | Yes    |
| Invite members                                                  | Yes   | Yes   | No              | No     |
| Change non-owner member roles                                   | Yes   | Yes   | No              | No     |
| Remove non-owner members                                        | Yes   | Yes   | No              | No     |
| Configure organization settings and data controls               | Yes   | Yes   | No              | No     |
| View subscription, usage, and billing information               | Yes   | Yes   | Yes             | No     |
| Update billing details, tax ID information, and payment methods | Yes   | Yes   | Yes             | No     |
| Cancel the subscription                                         | Yes   | No    | No              | No     |
| Transfer ownership                                              | Yes   | No    | No              | No     |

### Owner {#role-owner}

An owner has full control over the organization, including:

- Invite and remove members
- Assign and change member roles
- Manage billing, payment methods, and invoices
- Configure data-sharing policies
- Disable Zed's collaborative features
- Control whether members can use Zed-hosted models and Zed's edit predictions
- Transfer ownership to another member

### Admin {#role-admin}

Admins can manage members, roles, organization settings, data controls, and
billing. They have the same capabilities as the Owner, except they cannot:

- Cancel the subscription
- Transfer organization ownership

This role is suited for team leads or managers who handle day-to-day
member access and organization settings.

### Billing Manager {#role-billing-manager}

Billing Managers can view subscription usage, update billing details and tax ID
information, update payment methods, and access invoice history.

This role does not count toward paid Business seats. It also does not include
Zed-hosted AI models or Edit Predictions through the Business subscription.
Billing Managers cannot invite or remove members, change member roles, configure
organization settings or data controls, cancel the subscription, or transfer
ownership.

### Member {#role-member}

Members have standard access to Zed through the Business subscription. They
cannot access billing or organization settings.

## Managing User Roles {#managing-users}

Owners and Admins can manage organization members from the Zed dashboard within
the Members page.

### Inviting Members {#inviting-members}

1. On the Members page, select **+ Invite Member**.
2. Enter the member's company email address and choose a role.
3. The invitee receives an email with instructions to join. After
   accepting, they authenticate via GitHub.

### Changing a Member's Role {#changing-roles}

1. On the Members page, find the member. You can filter by role or
   search by name.
2. Open the three-dot menu and select a new role.

### Removing a Member {#removing-members}

1. On the Members page, find the member.
2. Select **Remove** and confirm.

Removing a member removes their access to organization settings and any organization-managed features. They can continue using Zed on their own.
