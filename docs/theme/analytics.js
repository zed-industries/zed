const amplitudeKey = document.querySelector(
  'meta[name="amplitude-key"]',
)?.content;
const consentInstance = document.querySelector(
  'meta[name="consent-io-instance"]',
)?.content;

document.addEventListener("DOMContentLoaded", () => {
  if (consentInstance.length === 0) return;
  const { getOrCreateConsentRuntime } = window.c15t;

  const { consentStore } = getOrCreateConsentRuntime({
    mode: "c15t",
    backendURL: consentInstance,
    consentCategories: ["necessary", "measurement"],
    scripts: [
      {
        id: "amplitude",
        src: `https://cdn.amplitude.com/script/${amplitudeKey}.js`,
        category: "measurement",
        onLoad: () => {
          window.amplitude.init(amplitudeKey, {
            fetchRemoteConfig: true,
            autocapture: true,
          });
        },
      },
    ],
  });

  const banner = document.getElementById("c15t-banner");
  const configureSection = document.getElementById("c15t-configure-section");
  const configureBtn = document.getElementById("c15t-configure-btn");
  const measurementToggle = document.getElementById("c15t-toggle-measurement");

  const toggleConfigureMode = () => {
    const currentConsents = consentStore.getState().consents;
    measurementToggle.checked = currentConsents
      ? (currentConsents.measurement ?? false)
      : false;
    configureSection.style.display = "flex";
    configureBtn.innerHTML = "Save";
    configureBtn.className = "c15t-button secondary";
    configureBtn.title = "";
  };

  consentStore.subscribe((state) => {
    const hideBanner = state.activeUI === "none";
    banner.style.display = hideBanner ? "none" : "block";

    if (state.activeUI === "dialog") {
      toggleConfigureMode();
    }
  });

  configureBtn.addEventListener("click", () => {
    if (consentStore.getState().activeUI === "dialog") {
      consentStore
        .getState()
        .setConsent("measurement", measurementToggle.checked);
    } else {
      toggleConfigureMode();
    }
  });

  document.getElementById("c15t-accept").addEventListener("click", () => {
    consentStore.getState().saveConsents("all");
  });

  document.getElementById("c15t-decline").addEventListener("click", () => {
    consentStore.getState().saveConsents("necessary");
  });

  document
    .getElementById("c15t-manage-consent-btn")
    .addEventListener("click", () => {
      consentStore.getState().setActiveUI("dialog");
    });
});
