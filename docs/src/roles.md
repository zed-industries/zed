---
title: Roles - Zed
description: Understand Zed's organization roles and what each role can access, manage, and configure.
---

# Roles

This page covers how roles work and what each role can do.

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

1. On the Members page, select **+ Invite Member**.
2. Enter the member's company email address and choose a role.
3. The invitee will receive an email with instructions to join (they'll be asked post-acceptance to authenticate via GitHub).

### Changing a Member's Role {#changing-roles}

1. On the Members page, find the member in the member list. You can filter by role or search by name.
2. In the three-dot menu, update the member's role.

### Removing a Member {#removing-members}

1. On the Members page, find the member in the member list.
2. Select **Remove** and confirm.

Removing a member revokes their access to Zed-hosted models and organization-managed features at the end of the current billing cycle.
