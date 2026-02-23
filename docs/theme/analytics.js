const amplitudeKey = "123";
if (amplitudeKey && amplitudeKey.indexOf("#") === -1) {
  import("https://esm.sh/c15t@1.8.3").then(
    ({ configureConsentManager, createConsentManagerStore }) => {
      const manager = configureConsentManager({
        mode: "offline",
      });
      const store = createConsentManagerStore(manager, {
        initialGdprTypes: ["necessary", "measurement"],
        ignoreGeoLocation: true,
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
      const configureSection = document.getElementById(
        "c15t-configure-section",
      );
      const configureBtn = document.getElementById("c15t-configure-btn");
      const measurementToggle = document.getElementById(
        "c15t-toggle-measurement",
      );
      let isConfiguring = false;

      function syncBanner(state) {
        banner.style.display = state.showPopup ? "block" : "none";
      }

      store.subscribe((state) => syncBanner(state));
      syncBanner(store.getState());

      configureBtn.addEventListener("click", () => {
        if (isConfiguring) {
          store.getState().setConsent("measurement", measurementToggle.checked);
        } else {
          isConfiguring = true;
          const currentConsents = store.getState().consents;
          measurementToggle.checked = currentConsents
            ? (currentConsents.measurement ?? false)
            : false;
          configureSection.style.display = "flex";
          configureBtn.innerHTML = "Save";
          configureBtn.className = "c15t-button secondary";
          configureBtn.title = "";
        }
      });

      document.getElementById("c15t-accept").addEventListener("click", () => {
        store.getState().setConsent("measurement", true);
      });

      document.getElementById("c15t-decline").addEventListener("click", () => {
        store.getState().setConsent("measurement", false);
      });
    },
  );
}
