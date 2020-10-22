define([
    "jquery",
    "lib/bootstrap.bundle.min",
    "lib/lodash",
    "text!app/templates/main.html",
    "i18n!app/nls/main",
    "app/settings",
    "app/setupWizard",
    "app/monitor",
    "app/driverList",
    "json!../../session",
    "text!../../version",
    "app/monitor",
    "js/lib/lobibox.min.js",
    "css!js/lib/lobibox.min.css"


], function ($, bootstrap, _, mainTemplate, i18n, Settings, SetupWizard, Monitor, driverList, session, version) {
    $(function () {

        var compiledTemplate = _.template(mainTemplate);
        var template = compiledTemplate(i18n);

        $("#bodyContent").append(template);
        $(document).ready(() => {
            $('#loading').remove();

            try {
                var settings = new Settings();
                var wizard = new SetupWizard(settings);
                var monitor = new Monitor(settings);
            } catch (error) {
                Lobibox.notify("error", {
                    rounded: true,
                    delay : -1,
                    delayIndicator: false,
                    sound: false,
                    position: "bottom left",
                    iconSource: 'fontAwesome',
                    msg: error.stack,
                    closable: true,
                    messageHeight: 250,
                });
            }                     

            $("#bodyContent").show();

            if (session.setupWizard) {
                wizard.showWizard();
            }

            $("#runSetupWizard").click(() => {
                wizard.showWizard();
            })

            $("#addFirewallRules").click(() => {
                $.get("firewall-rules/add", undefined, (res) => {
                    if (res == 0) {
                        Lobibox.notify("success", {
                            size: "mini",
                            rounded: true,
                            delayIndicator: false,
                            sound: false,
                            msg: i18n.firewallSuccess
                        })
                    }
                })
            })

            $("#removeFirewallRules").click(() => {
                $.get("firewall-rules/remove", undefined, (res) => {
                    if (res == 0) {
                        Lobibox.notify("success", {
                            size: "mini",
                            rounded: true,
                            delayIndicator: false,
                            sound: false,
                            msg: i18n.firewallSuccess
                        })
                    }
                })
            })

            $("#checkForUpdates").click(() => {
                $.get("https://api.github.com/repos/JackD83/ALVR/releases/latest", (data) => {
                    if(version == data.tag_name.match(/\d+.\d+.\d+/)[0]) {
                        Lobibox.notify("success", {
                            size: "mini",
                            rounded: true,
                            delayIndicator: false,
                            sound: false,
                            msg: i18n.noNeedForUpdate
                        })
                    } else {
                        Lobibox.notify("warning", {
                            size: "mini",
                            rounded: true,
                            delayIndicator: false,
                            sound: false,
                            msg: i18n.needUpdateClickForMore,
                            onClick: function(){
                                $.ajax({
                                    headers: { 
                                        'Accept': 'application/json',
                                        'Content-Type': 'application/json' 
                                    },
                                    'type': 'POST',
                                    'url': "/open",
                                    'data': JSON.stringify("https://github.com/JackD83/ALVR/releases/latest"),
                                    'dataType': 'JSON'
                                })
                            }
                        })
                    }
                })
            })

            $("#version").text("v" + version);

            driverList.fillDriverList("registeredDriversInst");


        });
    });
});