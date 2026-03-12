---
title: Organizations and Roles - Zed
description: Manage your Zed organization, invite members, and assign roles to control access to billing, models, and data sharing settings.
---

# Organizations and Roles

Zed Business plans let you create an organization, invite members, and
control what they can access. This page covers how roles work and what
each role can do.

> For details on Business plan pricing, see
> [Plans and Usage](./ai/plans-and-usage.md). For billing management,
> see [Billing](./ai/billing.md).

## Role Types {#roles}

Every member of an organization is assigned one of three roles:

| Role       | Description                                            |
| ---------- | ------------------------------------------------------ |
| **Owner**  | Full control, including billing and ownership transfer |
| **Admin**  | Full control, except billing                           |
| **Member** | Standard access, no privileged actions                 |

### Owner {#role-owner}

An owner has full control over the organization, including:

- Invite new users
- Assign and changeusers' roles
- Manage billing, payment methods, and invoices
- Configure data-sharing policies
- Control how their organization engages with Zed's AI features
- Transferring ownership to another member

### Admin {#role-admin}

Admins have the same capabilities as the Owner, except they cannot:

- Access or modify billing settings
- Transfer organization ownership

Admins are intended for team leads or managers who need to manage
day-to-day member access without handling payment details.

### Member {#role-member}

Members have standard access to Zed. They cannot access billing or
organization settings.

## Managing Members {#managing-members}

Owners and Admins can manage organization members from the Zed
dashboard within the Members page.

### Inviting Members {#inviting-members}

1. Navigate to the Members page on your organization's dashboard.
2. Select **+ Invite Member**.
3. Enter the member's company email address and choose a role.
4. The invitee will receive an email with instructions to join (they'll be asked post-acceptance to authenticate via GitHub).

### Changing a Member's Role {#changing-roles}

1. Navigate to the Members page on your organization's dashboard.
2. Find the member in the member list. You can filter by role or search by name.
3. In the three-dot menu, you can update the member's role.

### Removing a Member {#removing-members}

1. Navigate to your organization settings on the dashboard.
2. Find the member in the member list.
3. Select **Remove** and confirm.

Removing a member revokes their access to Zed-hosted models and
organization-managed features at the end of the current billing cycle.

## Organization Policies {#policies}

Owners and Admins can configure organization-wide policies that apply
to all members.

### Model Access {#model-access}

Control which Zed-hosted models are available to organization members.
Members can only use models that the organization has enabled.

### Data Sharing {#data-sharing}

Organizations can restrict data sharing with Zed for all members. When
restricted, members cannot:

- Submit agent thread feedback
- Rate Edit Predictions

For more on Zed's data practices, see
[Privacy and Security](./ai/privacy-and-security.md).

## Permissions Reference {#permissions-reference}

| Action                          | Owner | Admin | Member |
| ------------------------------- | :---: | :---: | :----: |
| Use Zed-hosted models           |   ✓   |   ✓   |   ✓    |
| Use Edit Predictions            |   ✓   |   ✓   |   ✓    |
| Invite and remove members       |   ✓   |   ✓   |        |
| Assign and change roles         |   ✓   |   ✓   |        |
| Manage model availability       |   ✓   |   ✓   |        |
| Configure data-sharing policies |   ✓   |   ✓   |        |
| Manage billing and payment      |   ✓   |       |        |
| Transfer ownership              |   ✓   |       |        |
