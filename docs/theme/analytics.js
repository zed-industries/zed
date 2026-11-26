const amplitudeKey = document.querySelector(
  'meta[name="amplitude-key"]',
)?.content;
const consentInstance = document.querySelector(
  'meta[name="consent-io-instance"]',
)?.content;

document.addEventListener("DOMContentLoaded", () => {
  if (!consentInstance || consentInstance.length === 0) return;

  // Calls queue on this array until the c15t script tag replaces it with the
  // API object, so the banner works before and after the tag has loaded.
  window.c15t = window.c15t || [];
  const c15t = (...call) => window.c15t.push(call);

  // The banner is the markup in index.hbs, so c15t's stock UI stays off.
  c15t("config", {
    ui: false,
    consentCategories: ["necessary", "measurement", "marketing"],
    storageConfig: {
      crossSubdomain: true,
    },
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
  const configureIcon = configureBtn.innerHTML;
  const measurementToggle = document.getElementById("c15t-toggle-measurement");
  const marketingToggle = document.getElementById("c15t-toggle-marketing");
  let isConfiguring = false;

  // c15t reports which surface is due: "banner", "dialog" (our configure
  // view) or "none".
  c15t("on", "ui", (surface) => {
    const { promptRequirement } = window.c15t.getSnapshot();
    isConfiguring = surface === "dialog";
    banner.style.display =
      surface === "none" ||
      (surface === "banner" && promptRequirement.kind === "none")
        ? "none"
        : "block";

    measurementToggle.checked = window.c15t.has("measurement");
    marketingToggle.checked = window.c15t.has("marketing");
    configureSection.style.display = isConfiguring ? "flex" : "none";
    configureBtn.innerHTML = isConfiguring ? "Save" : configureIcon;
    configureBtn.className = isConfiguring
      ? "c15t-button secondary"
      : "c15t-button icon";
    configureBtn.title = isConfiguring ? "" : "Configure";
  });

  configureBtn.addEventListener("click", () => {
    if (isConfiguring) {
      c15t("save", {
        measurement: measurementToggle.checked,
        marketing: marketingToggle.checked,
      });
    } else {
      c15t("openDialog");
    }
  });

  document
    .getElementById("c15t-accept")
    .addEventListener("click", () => c15t("acceptAll"));

  document
    .getElementById("c15t-decline")
    .addEventListener("click", () => c15t("rejectAll"));

  document
    .getElementById("c15t-manage-consent-btn")
    .addEventListener("click", () => c15t("openDialog"));

  // Inth serves this file with our backend URL already configured.
  const script = document.createElement("script");
  script.src = `${consentInstance.replace(/\/+$/, "")}/c15t.js`;
  script.defer = true;
  document.head.appendChild(script);
});
