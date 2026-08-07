(() => {
  const legacyAnchors = {
    "#extension-publishing-prerequisites": {
      page: "publishing/prerequisites",
    },
    "#extension-license-requirements": {
      page: "publishing/license-requirements",
    },
    "#forking-and-cloning-the-repo": {
      page: "publishing/publishing-guide",
      hash: "#forking-and-cloning-the-repo",
    },
    "#publishing-your-extension": {
      page: "publishing/publishing-guide",
    },
    "#updating-an-extension": {
      page: "publishing/publishing-guide",
      hash: "#updating-an-extension",
    },
  };

  const redirectLegacyAnchor = () => {
    const destination = legacyAnchors[window.location.hash];
    const pathMatch = window.location.pathname.match(
      /^(.*\/extensions\/)developing-extensions(?:\.html)?\/?$/,
    );
    if (!destination || !pathMatch) return;

    const destinationUrl = new URL(window.location.href);
    destinationUrl.pathname = `${pathMatch[1]}${destination.page}.html`;
    destinationUrl.hash = destination.hash ?? "";
    window.location.replace(destinationUrl);
  };

  redirectLegacyAnchor();
  window.addEventListener("hashchange", redirectLegacyAnchor);
})();
