# RocketChat Roles

- export .env and run

```shell
---- Sessions ----
userId: G7RXKXa3w9pMn3RMt, loginToken: mtSrqXM/nGJg21/bKnhzq20OaKSKHEedozGrrJUzW40=, loginAt: 2024-05-03 15:57:39.484 UTC, role: admin, roles: ["user", "admin"]
userId: G7RXKXa3w9pMn3RMt, loginToken: mtSrqXM/nGJg21/bKnhzq20OaKSKHEedozGrrJUzW40=, loginAt: 2024-05-03 16:16:20.716 UTC, role: admin, roles: ["user", "admin"]
userId: G7RXKXa3w9pMn3RMt, loginToken: mtSrqXM/nGJg21/bKnhzq20OaKSKHEedozGrrJUzW40=, loginAt: 2024-05-03 16:21:30.714 UTC, role: admin, roles: ["user", "admin"]
userId: G7RXKXa3w9pMn3RMt, loginToken: mtSrqXM/nGJg21/bKnhzq20OaKSKHEedozGrrJUzW40=, loginAt: 2024-05-03 15:57:39.484 UTC, role: admin, roles: ["user", "admin"]
userId: G7RXKXa3w9pMn3RMt, loginToken: mtSrqXM/nGJg21/bKnhzq20OaKSKHEedozGrrJUzW40=, loginAt: 2024-05-03 16:16:20.716 UTC, role: admin, roles: ["user", "admin"]
---- Roles ----
name: admin, scope: Users
name: moderator, scope: Subscriptions
name: leader, scope: Subscriptions
name: owner, scope: Subscriptions
name: user, scope: Users
name: bot, scope: Users
name: app, scope: Users
name: guest, scope: Users
name: anonymous, scope: Users
name: livechat-agent, scope: Users
name: livechat-manager, scope: Users
name: livechat-monitor, scope: Users
---- Permissions ----
id: access-permissions, roles: ["admin"]
id: access-marketplace, roles: ["admin", "user"]
id: access-setting-permissions, roles: ["admin"]
id: add-oauth-service, roles: ["admin"]
id: add-user-to-joined-room, roles: ["admin", "owner", "moderator"]
id: add-user-to-any-c-room, roles: ["admin"]
id: kick-user-from-any-c-room, roles: ["admin"]
id: api-bypass-rate-limit, roles: ["admin", "bot", "app"]
id: archive-room, roles: ["admin", "owner"]
id: assign-admin-role, roles: ["admin"]
id: assign-roles, roles: ["admin"]
id: ban-user, roles: ["admin", "owner", "moderator"]
id: bulk-register-user, roles: ["admin"]
id: change-livechat-room-visitor, roles: ["admin", "livechat-manager", "livechat-agent"]
id: create-c, roles: ["admin", "user", "bot", "app"]
id: create-d, roles: ["admin", "user", "bot", "app"]
id: create-p, roles: ["admin", "user", "bot", "app"]
id: create-personal-access-tokens, roles: ["admin", "user"]
id: create-user, roles: ["admin"]
id: clean-channel-history, roles: ["admin"]
id: delete-c, roles: ["admin", "owner"]
id: delete-d, roles: ["admin"]
id: delete-message, roles: ["admin", "owner", "moderator"]
id: delete-own-message, roles: ["admin", "user"]
id: delete-p, roles: ["admin", "owner"]
id: delete-user, roles: ["admin"]
id: edit-message, roles: ["admin", "owner", "moderator"]
id: edit-other-user-active-status, roles: ["admin"]
id: edit-other-user-info, roles: ["admin"]
id: edit-other-user-password, roles: ["admin"]
id: edit-other-user-avatar, roles: ["admin"]
id: edit-other-user-e2ee, roles: ["admin"]
id: edit-other-user-totp, roles: ["admin"]
id: edit-privileged-setting, roles: ["admin"]
id: edit-room, roles: ["admin", "owner", "moderator"]
id: edit-room-avatar, roles: ["admin", "owner", "moderator"]
id: edit-room-retention-policy, roles: ["admin"]
id: force-delete-message, roles: ["admin", "owner"]
id: join-without-join-code, roles: ["admin", "bot", "app"]
id: leave-c, roles: ["admin", "user", "bot", "anonymous", "app"]
id: leave-p, roles: ["admin", "user", "bot", "anonymous", "app"]
id: logout-other-user, roles: ["admin"]
id: manage-assets, roles: ["admin"]
id: manage-email-inbox, roles: ["admin"]
id: manage-emoji, roles: ["admin"]
id: manage-user-status, roles: ["admin"]
id: manage-outgoing-integrations, roles: ["admin"]
id: manage-incoming-integrations, roles: ["admin"]
id: manage-own-outgoing-integrations, roles: ["admin"]
id: manage-own-incoming-integrations, roles: ["admin"]
id: manage-oauth-apps, roles: ["admin"]
id: manage-selected-settings, roles: ["admin"]
id: mention-all, roles: ["admin", "owner", "moderator", "user"]
id: mention-here, roles: ["admin", "owner", "moderator", "user"]
id: mute-user, roles: ["admin", "owner", "moderator"]
id: remove-user, roles: ["admin", "owner", "moderator"]
id: run-import, roles: ["admin"]
id: run-migration, roles: ["admin"]
id: set-moderator, roles: ["admin", "owner"]
id: set-owner, roles: ["admin", "owner"]
id: send-many-messages, roles: ["admin", "bot", "app"]
id: set-leader, roles: ["admin", "owner"]
id: unarchive-room, roles: ["admin"]
id: view-c-room, roles: ["admin", "user", "bot", "app", "anonymous"]
id: user-generate-access-token, roles: ["admin"]
id: view-d-room, roles: ["admin", "user", "bot", "app", "guest"]
id: view-device-management, roles: ["admin"]
id: view-engagement-dashboard, roles: ["admin"]
id: view-full-other-user-info, roles: ["admin"]
id: view-history, roles: ["admin", "user", "anonymous"]
id: view-joined-room, roles: ["guest", "bot", "app", "anonymous"]
id: view-join-code, roles: ["admin"]
id: view-logs, roles: ["admin"]
id: view-other-user-channels, roles: ["admin"]
id: view-p-room, roles: ["admin", "user", "anonymous", "guest"]
id: view-privileged-setting, roles: ["admin"]
id: view-room-administration, roles: ["admin"]
id: view-statistics, roles: ["admin"]
id: view-user-administration, roles: ["admin"]
id: preview-c-room, roles: ["admin", "user", "anonymous"]
id: view-outside-room, roles: ["admin", "owner", "moderator", "user"]
id: view-broadcast-member-list, roles: ["admin", "owner", "moderator"]
id: call-management, roles: ["admin", "owner", "moderator", "user"]
id: create-invite-links, roles: ["admin", "owner", "moderator"]
id: view-l-room, roles: ["livechat-manager", "livechat-monitor", "livechat-agent", "admin"]
id: view-livechat-manager, roles: ["livechat-manager", "livechat-monitor", "admin"]
id: view-omnichannel-contact-center, roles: ["livechat-manager", "livechat-agent", "livechat-monitor", "admin"]
id: edit-omnichannel-contact, roles: ["livechat-manager", "livechat-agent", "admin"]
id: view-livechat-rooms, roles: ["livechat-manager", "livechat-monitor", "admin"]
id: close-livechat-room, roles: ["livechat-manager", "livechat-monitor", "livechat-agent", "admin"]
id: close-others-livechat-room, roles: ["livechat-manager", "livechat-monitor", "admin"]
id: on-hold-livechat-room, roles: ["livechat-manager", "livechat-monitor", "livechat-agent", "admin"]
id: on-hold-others-livechat-room, roles: ["livechat-manager", "livechat-monitor", "admin"]
id: save-others-livechat-room-info, roles: ["livechat-manager", "livechat-monitor", "admin"]
id: remove-closed-livechat-rooms, roles: ["livechat-manager", "livechat-monitor", "admin"]
id: view-livechat-analytics, roles: ["livechat-manager", "livechat-monitor", "admin"]
id: view-livechat-queue, roles: ["livechat-manager", "livechat-monitor", "livechat-agent", "admin"]
id: transfer-livechat-guest, roles: ["livechat-manager", "livechat-monitor", "admin"]
id: manage-livechat-managers, roles: ["livechat-manager", "admin"]
id: manage-livechat-agents, roles: ["livechat-manager", "livechat-monitor", "admin"]
id: manage-livechat-departments, roles: ["livechat-manager", "livechat-monitor", "admin"]
id: view-livechat-departments, roles: ["livechat-manager", "livechat-monitor", "admin"]
id: add-livechat-department-agents, roles: ["livechat-manager", "livechat-monitor", "admin"]
id: view-livechat-current-chats, roles: ["livechat-manager", "livechat-monitor", "admin"]
id: view-livechat-real-time-monitoring, roles: ["livechat-manager", "livechat-monitor", "admin"]
id: view-livechat-triggers, roles: ["livechat-manager", "admin"]
id: view-livechat-customfields, roles: ["livechat-manager", "admin"]
id: view-livechat-installation, roles: ["livechat-manager", "admin"]
id: view-livechat-appearance, roles: ["livechat-manager", "admin"]
id: view-livechat-webhooks, roles: ["livechat-manager", "admin"]
id: view-livechat-business-hours, roles: ["livechat-manager", "livechat-monitor", "admin"]
id: view-livechat-room-closed-same-department, roles: ["livechat-manager", "livechat-monitor", "admin"]
id: view-livechat-room-closed-by-another-agent, roles: ["livechat-manager", "livechat-monitor", "admin"]
id: view-livechat-room-customfields, roles: ["livechat-manager", "livechat-monitor", "livechat-agent", "admin"]
id: edit-livechat-room-customfields, roles: ["livechat-manager", "livechat-monitor", "livechat-agent", "admin"]
id: send-omnichannel-chat-transcript, roles: ["livechat-manager", "admin"]
id: mail-messages, roles: ["admin"]
id: toggle-room-e2e-encryption, roles: ["owner", "admin"]
id: message-impersonate, roles: ["bot", "app"]
id: create-team, roles: ["admin", "user"]
id: delete-team, roles: ["admin", "owner"]
id: convert-team, roles: ["admin", "owner"]
id: edit-team, roles: ["admin", "owner"]
id: add-team-member, roles: ["admin", "owner", "moderator"]
id: edit-team-member, roles: ["admin", "owner", "moderator"]
id: add-team-channel, roles: ["admin", "owner", "moderator"]
id: edit-team-channel, roles: ["admin", "owner", "moderator"]
id: remove-team-channel, roles: ["admin", "owner", "moderator"]
id: view-all-team-channels, roles: ["admin", "owner"]
id: view-all-teams, roles: ["admin"]
id: remove-closed-livechat-room, roles: ["livechat-manager", "admin"]
id: remove-livechat-department, roles: ["livechat-manager", "admin"]
id: manage-voip-call-settings, roles: ["livechat-manager", "admin"]
id: manage-voip-contact-center-settings, roles: ["livechat-manager", "admin"]
id: manage-agent-extension-association, roles: ["admin"]
id: view-agent-extension-association, roles: ["livechat-manager", "admin", "livechat-agent"]
id: inbound-voip-calls, roles: ["livechat-agent"]
id: manage-apps, roles: ["admin"]
id: post-readonly, roles: ["admin", "owner", "moderator"]
id: set-readonly, roles: ["admin", "owner"]
id: set-react-when-readonly, roles: ["admin", "owner"]
id: manage-cloud, roles: ["admin"]
id: manage-sounds, roles: ["admin"]
id: access-mailer, roles: ["admin"]
id: pin-message, roles: ["owner", "moderator", "admin"]
id: mobile-upload-file, roles: ["user", "admin"]
id: send-mail, roles: ["admin"]
id: view-federation-data, roles: ["admin"]
id: add-all-to-room, roles: ["admin"]
id: get-server-info, roles: ["admin"]
id: register-on-cloud, roles: ["admin"]
id: test-admin-options, roles: ["admin"]
id: test-push-notifications, roles: ["admin", "user"]
id: sync-auth-services-users, roles: ["admin"]
id: restart-server, roles: ["admin"]
id: remove-slackbridge-links, roles: ["admin"]
id: view-import-operations, roles: ["admin"]
id: clear-oembed-cache, roles: ["admin"]
id: videoconf-ring-users, roles: ["admin", "owner", "moderator", "user"]
id: view-moderation-console, roles: ["admin"]
id: manage-moderation-actions, roles: ["admin"]
id: bypass-time-limit-edit-and-delete, roles: ["bot", "app"]
id: auto-translate, roles: ["admin"]
id: start-discussion, roles: ["admin", "user", "guest", "app"]
id: start-discussion-other-user, roles: ["admin", "user", "owner", "app"]
id: view-canned-responses, roles: ["livechat-agent", "livechat-monitor", "livechat-manager", "admin"]
id: request-pdf-transcript, roles: ["admin", "livechat-manager", "livechat-monitor", "livechat-agent"]
id: outbound-voip-calls, roles: ["admin", "livechat-manager"]
id: spy-voip-calls, roles: ["admin", "livechat-manager", "livechat-monitor"]
id: manage-livechat-canned-responses, roles: ["admin", "livechat-manager", "livechat-monitor"]
id: view-all-canned-responses, roles: ["livechat-manager", "admin"]
id: manage-livechat-sla, roles: ["admin", "livechat-manager"]
id: manage-livechat-priorities, roles: ["admin", "livechat-manager"]
id: manage-livechat-tags, roles: ["admin", "livechat-manager"]
id: manage-livechat-monitors, roles: ["admin", "livechat-manager"]
id: manage-livechat-units, roles: ["admin", "livechat-manager"]
id: view-livechat-reports, roles: ["admin", "livechat-manager", "livechat-monitor"]
id: view-agent-canned-responses, roles: ["livechat-agent"]
id: save-canned-responses, roles: ["livechat-agent", "livechat-monitor", "livechat-manager", "admin"]
id: save-all-canned-responses, roles: ["livechat-manager", "admin"]
id: save-department-canned-responses, roles: ["livechat-monitor"]
id: remove-canned-responses, roles: ["livechat-agent", "livechat-monitor", "livechat-manager", "admin"]
```
