use yew::prelude::*;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    html! {
        <div class="auth-container">
            <div class="auth-icon">
                <span class="material-icons">{"lock"}</span>
            </div>
            <h2>{"Login"}</h2>
            <form>
                <div class="form-field">
                    <label for="username">{"Username"}</label>
                    <input type="text" id="username" name="username" />
                </div>
                <div class="form-field">
                    <label for="password">{"Password"}</label>
                    <input type="password" id="password" name="password" />
                </div>
                <button type="submit" class="auth-button">
                    {"Login"}
                </button>
            </form>
        </div>
    }
}