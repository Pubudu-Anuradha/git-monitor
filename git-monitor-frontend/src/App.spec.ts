import { mount } from "@vue/test-utils";
import AppVue from "./App.vue";

it("should render", () => {
  // Try to render <App /> here
  mount(AppVue).text().includes("Git Monitor");
});
