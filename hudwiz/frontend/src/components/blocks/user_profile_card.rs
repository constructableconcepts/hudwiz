use yew::prelude::*;
use material_yew::Icon;

#[function_component(UserProfileCard)]
pub fn user_profile_card() -> Html {
    html! {
        <div class="user-profile-card">
            <div class="card-header">
                <div class="avatar-container">
                    <Icon icon={{"account_circle"}} />
                </div>
            </div>
            <div class="card-body">
                <h3 class="user-name">{"Jules Verne"}</h3>
                <p class="user-title">{"Software Engineer"}</p>
                <p class="user-bio">
                    {"An agent of change, crafting digital experiences one line of code at a time."}
                </p>
            </div>
            <div class="card-footer">
                <button class="button">{"View Profile"}</button>
            </div>
        </div>
    }
}