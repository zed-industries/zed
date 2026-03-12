---
title: Roles - Zed
description: Understand Zed's organization roles and what each role can access, manage, and configure.
---

# Roles

Every member of a Zed organization is assigned a role that determines
what they can access and configure.

## Role Types {#roles}

Every member of an organization is assigned one of three roles:

| Role       | Description                                            |
| ---------- | ------------------------------------------------------ |
| **Owner**  | Full control, including billing and ownership transfer |
| **Admin**  | Full control, except billing                           |
| **Member** | Standard access, no privileged actions                 |

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

Admins have the same capabilities as the Owner, except they cannot:

- Access or modify billing settings
- Transfer organization ownership

This role is suited for team leads or managers who handle day-to-day
member access without needing visibility into payment details.

### Member {#role-member}

Members have standard access to Zed. They cannot access billing or
organization settings.

## Managing User Roles {#managing-users}

Owners and Admins can manage organization members from the Zed
dashboard within the Members page.

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
