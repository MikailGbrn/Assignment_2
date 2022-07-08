import com.kms.katalon.core.main.TestCaseMain
import com.kms.katalon.core.logging.KeywordLogger
import com.kms.katalon.core.testcase.TestCaseBinding
import com.kms.katalon.core.driver.internal.DriverCleanerCollector
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.webui.contribution.WebUiDriverCleaner
import com.kms.katalon.core.mobile.contribution.MobileDriverCleaner
import com.kms.katalon.core.cucumber.keyword.internal.CucumberDriverCleaner
import com.kms.katalon.core.windows.keyword.contribution.WindowsDriverCleaner
import com.kms.katalon.core.testng.keyword.internal.TestNGDriverCleaner


DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.webui.contribution.WebUiDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.mobile.contribution.MobileDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.cucumber.keyword.internal.CucumberDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.windows.keyword.contribution.WindowsDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.testng.keyword.internal.TestNGDriverCleaner())


RunConfiguration.setExecutionSettingFile('C:\\Users\\095097\\AppData\\Local\\Temp\\Katalon\\20220708_165954\\execution.properties')

TestCaseMain.beforeStart()

        TestCaseMain.runWSVerificationScript(new TestCaseBinding('',[:]), 'import static org.assertj.core.api.Assertions.*\n\nimport com.kms.katalon.core.testobject.RequestObject\nimport com.kms.katalon.core.testobject.ResponseObject\nimport com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS\nimport com.kms.katalon.core.webservice.verification.WSResponseManager\n\nimport groovy.json.JsonSlurper\nimport internal.GlobalVariable as GlobalVariable\n\nRequestObject request = WSResponseManager.getInstance().getCurrentRequest()\n\nResponseObject response = WSResponseManager.getInstance().getCurrentResponse()\n\n\nWS.verifyResponseStatusCode(response, 200)\n\nassertThat(response.getStatusCode()).isEqualTo(200)\n\nWS.verifyElementPropertyValue(response, \'[10].postId\', \'3\')\nWS.verifyElementPropertyValue(response, \'[10].id\', \'11\')\nWS.verifyElementPropertyValue(response, \'[10].name\', \'fugit labore quia mollitia quas deserunt nostrum sunt\')\nWS.verifyElementPropertyValue(response, \'[10].email\', \'Veronica_Goodwin@timmothy.net\')\nWS.verifyElementPropertyValue(response, \'[10].body\', \'ut dolorum nostrum id quia aut est\\nfuga est inventore vel eligendi explicabo quis consectetur\\naut occaecati repellat id natus quo est\\nut blanditiis quia ut vel ut maiores ea\')\n\nWS.verifyElementPropertyValue(response, \'[19].postId\', \'4\')\nWS.verifyElementPropertyValue(response, \'[19].id\', \'20\')\nWS.verifyElementPropertyValue(response, \'[19].name\', \'molestias expedita iste aliquid voluptates\')\nWS.verifyElementPropertyValue(response, \'[19].email\', \'Mariana_Orn@preston.org\')\nWS.verifyElementPropertyValue(response, \'[19].body\', \'qui harum consequatur fugiat\\net eligendi perferendis at molestiae commodi ducimus\\ndoloremque asperiores numquam qui\\nut sit dignissimos reprehenderit tempore\')\n\n\nWS.verifyElementPropertyValue(response, \'[13].postId\', \'3\')\nWS.verifyElementPropertyValue(response, \'[13].id\', \'14\')\nWS.verifyElementPropertyValue(response, \'[13].name\', \'et officiis id praesentium hic aut ipsa dolorem repudiandae\')\nWS.verifyElementPropertyValue(response, \'[13].email\', \'Nathan@solon.io\')\nWS.verifyElementPropertyValue(response, \'[13].body\', \'vel quae voluptas qui exercitationem\\nvoluptatibus unde sed\\nminima et qui ipsam aspernatur\\nexpedita magnam laudantium et et quaerat ut qui dolorum\')\n\nWS.verifyElementPropertyValue(response, \'[31].postId\', \'7\')\nWS.verifyElementPropertyValue(response, \'[31].id\', \'32\')\nWS.verifyElementPropertyValue(response, \'[31].name\', \'dolorem architecto ut pariatur quae qui suscipit\')\nWS.verifyElementPropertyValue(response, \'[31].email\', \'Maria@laurel.name\')\nWS.verifyElementPropertyValue(response, \'[31].body\', \'nihil ea itaque libero illo\\nofficiis quo quo dicta inventore consequatur voluptas voluptatem\\ncorporis sed necessitatibus velit tempore\\nrerum velit et temporibus\')\n\n\nWS.verifyElementPropertyValue(response, \'[466].postId\', \'94\')\nWS.verifyElementPropertyValue(response, \'[466].id\', \'467\')\nWS.verifyElementPropertyValue(response, \'[466].name\', \'voluptas unde perferendis ut eaque dicta\')\nWS.verifyElementPropertyValue(response, \'[466].email\', \'Adrianna_Howell@molly.io\')\nWS.verifyElementPropertyValue(response, \'[466].body\', \'deleniti fuga hic autem\\nsed rerum non voluptate sit totam consequuntur illo\\nquasi quod aut ducimus dolore distinctio molestias\\nnon velit quis debitis cumque voluptas\')', FailureHandling.STOP_ON_FAILURE, true)

