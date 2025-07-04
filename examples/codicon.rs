#![allow(unused_must_use)]

use iced::{
    Center, Element, Length, Task, font,
    widget::{column, row},
};
use iced_fonts::{CODICON_FONT_BYTES, codicon::*};

pub fn main() -> iced::Result {
    iced::application("codicon", App::update, App::view).run_with(App::new)
}

#[derive(Default)]
struct App {}

#[derive(Debug, Clone, Copy)]
enum Message {
    FontLoaded(Result<(), font::Error>),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {},
            Task::batch(vec![
                font::load(CODICON_FONT_BYTES).map(Message::FontLoaded),
            ]),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::FontLoaded(result) => {
                dbg!(result);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![
                add(),
                lightbulb(),
                repo(),
                repo_forked(),
                git_pull_request(),
                record_keys(),
                tag(),
                person(),
                source_control(),
                mirror(),
                star_empty(),
                comment(),
                warning(),
                search(),
                sign_out(),
                sign_in(),
                eye(),
                circle_filled(),
                primitive_square(),
                edit(),
                info(),
                lock(),
                close(),
                sync(),
                desktop_download(),
                beaker(),
                vm(),
                file(),
            ]
            .padding(12)
            .spacing(20)
            .width(Length::Fill)
            .align_y(Center),
            row![
                ellipsis(),
                reply(),
                organization(),
                new_file(),
                new_folder(),
                trash(),
                history(),
                folder(),
                github(),
                terminal(),
                symbol_event(),
                error(),
                symbol_variable(),
                symbol_array(),
                symbol_namespace(),
                symbol_method(),
                symbol_boolean(),
                symbol_numeric(),
                symbol_structure(),
                symbol_parameter(),
                symbol_key(),
                go_to_file(),
                symbol_enum(),
                symbol_ruler(),
                activate_breakpoints(),
                archive(),
                arrow_both(),
                arrow_down(),
            ]
            .padding(12)
            .spacing(20)
            .width(Length::Fill)
            .align_y(Center),
            row![
                arrow_left(),
                arrow_right(),
                arrow_small_down(),
                arrow_small_left(),
                arrow_small_right(),
                arrow_small_up(),
                arrow_up(),
                bell(),
                bold(),
                book(),
                bookmark(),
                debug_breakpoint_conditional_unverified(),
                debug_breakpoint_conditional(),
                debug_breakpoint_data_unverified(),
                debug_breakpoint_data(),
                debug_breakpoint_log_unverified(),
                debug_breakpoint_log(),
                briefcase(),
                broadcast(),
                browser(),
                bug(),
                calendar(),
                case_sensitive(),
                check(),
                checklist(),
                chevron_down(),
                chevron_left(),
                chevron_right(),
            ]
            .padding(12)
            .spacing(20)
            .width(Length::Fill)
            .align_y(Center),
            row![
                chevron_up(),
                chrome_close(),
                chrome_maximize(),
                chrome_minimize(),
                chrome_restore(),
                circle(),
                circle_slash(),
                circuit_board(),
                clear_all(),
                clippy(),
                close_all(),
                cloud_download(),
                cloud_upload(),
                code(),
                collapse_all(),
                color_mode(),
                comment_discussion(),
                credit_card(),
                dash(),
                dashboard(),
                database(),
                debug_continue(),
                debug_disconnect(),
                debug_pause(),
                debug_restart(),
                debug_start(),
                debug_step_into(),
                debug_step_out(),
            ]
            .padding(12)
            .spacing(20)
            .width(Length::Fill)
            .align_y(Center),
            row![
                debug_step_over(),
                debug_stop(),
                debug(),
                device_camera_video(),
                device_camera(),
                device_mobile(),
                diff_added(),
                diff_ignored(),
                diff_modified(),
                diff_removed(),
                diff_renamed(),
                diff(),
                discard(),
                editor_layout(),
                empty_window(),
                exclude(),
                extensions(),
                eye_closed(),
                file_binary(),
                file_code(),
                file_media(),
                file_pdf(),
                file_submodule(),
                file_symlink_directory(),
                file_symlink_file(),
                file_zip(),
                files(),
                filter(),
            ]
            .padding(12)
            .spacing(20)
            .width(Length::Fill)
            .align_y(Center),
            row![
                flame(),
                fold_down(),
                fold_up(),
                fold(),
                folder_active(),
                folder_opened(),
                gear(),
                gift(),
                gist_secret(),
                git_commit(),
                git_compare(),
                git_merge(),
                github_action(),
                github_alt(),
                globe(),
                grabber(),
                graph(),
                gripper(),
                heart(),
                home(),
                horizontal_rule(),
                hubot(),
                inbox(),
                issue_reopened(),
                issues(),
                italic(),
                jersey(),
                json(),
            ]
            .padding(12)
            .spacing(20)
            .width(Length::Fill)
            .align_y(Center),
            row![
                kebab_vertical(),
                key(),
                law(),
                lightbulb_autofix(),
                link_external(),
                link(),
                list_ordered(),
                list_unordered(),
                live_share(),
                loading(),
                location(),
                mail_read(),
                mail(),
                markdown(),
                megaphone(),
                mention(),
                milestone(),
                mortar_board(),
                r#move(),
                multiple_windows(),
                mute(),
                no_newline(),
                note(),
                octoface(),
                open_preview(),
                package(),
                paintcan(),
                pin(),
            ]
            .padding(12)
            .spacing(20)
            .width(Length::Fill)
            .align_y(Center),
            row![
                play(),
                plug(),
                preserve_case(),
                preview(),
                project(),
                pulse(),
                question(),
                quote(),
                radio_tower(),
                reactions(),
                references(),
                refresh(),
                regex(),
                remote_explorer(),
                remote(),
                remove(),
                replace_all(),
                replace(),
                repo_clone(),
                repo_force_push(),
                repo_pull(),
                repo_push(),
                report(),
                request_changes(),
                rocket(),
                root_folder_opened(),
                root_folder(),
                rss(),
            ]
            .padding(12)
            .spacing(20)
            .width(Length::Fill)
            .align_y(Center),
            row![
                ruby(),
                save_all(),
                save_as(),
                save(),
                screen_full(),
                screen_normal(),
                search_stop(),
                server(),
                settings_gear(),
                settings(),
                shield(),
                smiley(),
                sort_precedence(),
                split_horizontal(),
                split_vertical(),
                squirrel(),
                star_full(),
                star_half(),
                symbol_class(),
                symbol_color(),
                symbol_constant(),
                symbol_enum_member(),
                symbol_field(),
                symbol_file(),
                symbol_interface(),
                symbol_keyword(),
                symbol_misc(),
                symbol_operator(),
            ]
            .padding(12)
            .spacing(20)
            .width(Length::Fill)
            .align_y(Center),
            row![
                symbol_property(),
                symbol_snippet(),
                tasklist(),
                telescope(),
                text_size(),
                three_bars(),
                thumbsdown(),
                thumbsup(),
                tools(),
                triangle_down(),
                triangle_left(),
                triangle_right(),
                triangle_up(),
                twitter(),
                unfold(),
                unlock(),
                unmute(),
                unverified(),
                verified(),
                versions(),
                vm_active(),
                vm_outline(),
                vm_running(),
                watch(),
                whitespace(),
                whole_word(),
                window(),
                word_wrap(),
            ]
            .padding(12)
            .spacing(20)
            .width(Length::Fill)
            .align_y(Center),
            row![
                zoom_in(),
                zoom_out(),
                list_filter(),
                list_flat(),
                list_selection(),
                list_tree(),
                debug_breakpoint_function_unverified(),
                debug_breakpoint_function(),
                debug_stackframe_active(),
                circle_small_filled(),
                debug_stackframe(),
                debug_breakpoint_unsupported(),
                symbol_string(),
                debug_reverse_continue(),
                debug_step_back(),
                debug_restart_frame(),
                debug_alt(),
                call_incoming(),
                call_outgoing(),
                menu(),
                expand_all(),
                feedback(),
                group_by_ref_type(),
                ungroup_by_ref_type(),
                account(),
                bell_dot(),
                debug_console(),
                library(),
            ]
            .padding(12)
            .spacing(20)
            .width(Length::Fill)
            .align_y(Center),
            row![
                output(),
                run_all(),
                sync_ignored(),
                pinned(),
                github_inverted(),
                server_process(),
                server_environment(),
                pass(),
                stop_circle(),
                play_circle(),
                record(),
                debug_alt_small(),
                vm_connect(),
                cloud(),
                merge(),
                export(),
                graph_left(),
                magnet(),
                notebook(),
                redo(),
                check_all(),
                pinned_dirty(),
                pass_filled(),
                circle_large_filled(),
                circle_large(),
                combine(),
                table(),
                variable_group(),
            ]
            .padding(12)
            .spacing(20)
            .width(Length::Fill)
            .align_y(Center),
            row![
                type_hierarchy(),
                type_hierarchy_sub(),
                type_hierarchy_super(),
                git_pull_request_create(),
                run_above(),
                run_below(),
                notebook_template(),
                debug_rerun(),
                workspace_trusted(),
                workspace_untrusted(),
                workspace_unknown(),
                terminal_cmd(),
                terminal_debian(),
                terminal_linux(),
                terminal_powershell(),
                terminal_tmux(),
                terminal_ubuntu(),
                terminal_bash(),
                arrow_swap(),
                copy(),
                person_add(),
                filter_filled(),
                wand(),
                debug_line_by_line(),
                inspect(),
                layers(),
                layers_dot(),
                layers_active(),
            ]
            .padding(12)
            .spacing(20)
            .width(Length::Fill)
            .align_y(Center),
            row![
                compass(),
                compass_dot(),
                compass_active(),
                azure(),
                issue_draft(),
                git_pull_request_closed(),
                git_pull_request_draft(),
                debug_all(),
                debug_coverage(),
                run_errors(),
                folder_library(),
                debug_continue_small(),
                beaker_stop(),
                graph_line(),
                graph_scatter(),
                pie_chart(),
                bracket_dot(),
                bracket_error(),
                lock_small(),
                azure_devops(),
                verified_filled(),
                newline(),
                layout(),
                layout_activitybar_left(),
                layout_activitybar_right(),
                layout_panel_left(),
                layout_panel_center(),
                layout_panel_justify(),
            ]
            .padding(12)
            .spacing(20)
            .width(Length::Fill)
            .align_y(Center),
            row![
                layout_panel_right(),
                layout_panel(),
                layout_sidebar_left(),
                layout_sidebar_right(),
                layout_statusbar(),
                layout_menubar(),
                layout_centered(),
                target(),
                indent(),
                record_small(),
                error_small(),
                arrow_circle_down(),
                arrow_circle_left(),
                arrow_circle_right(),
                arrow_circle_up(),
                layout_sidebar_right_off(),
                layout_panel_off(),
                layout_sidebar_left_off(),
                blank(),
                heart_filled(),
                map(),
                map_filled(),
                circle_small(),
                bell_slash(),
                bell_slash_dot(),
                comment_unresolved(),
                git_pull_request_go_to_changes(),
                git_pull_request_new_changes(),
            ]
            .padding(12)
            .spacing(20)
            .width(Length::Fill)
            .align_y(Center),
            row![
                search_fuzzy(),
                comment_draft(),
                send(),
                sparkle(),
                insert(),
                mic(),
                thumbsdown_filled(),
                thumbsup_filled(),
                coffee(),
                snake(),
                game(),
                vr(),
                chip(),
                piano(),
                music(),
                mic_filled(),
                git_fetch(),
                copilot(),
            ]
            .padding(12)
            .spacing(20)
            .width(Length::Fill)
            .align_y(Center),
        ]
        .into()
    }
}
