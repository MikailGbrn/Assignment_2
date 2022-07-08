import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

response = WS.sendRequest(findTestObject('Albums/GET All Albums'))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, '[19].userId', '2')
WS.verifyElementPropertyValue(response, '[19].id', '20')
WS.verifyElementPropertyValue(response, '[19].title', 'voluptas rerum iure ut enim')

WS.verifyElementPropertyValue(response, '[28].userId', '3')
WS.verifyElementPropertyValue(response, '[28].id', '29')
WS.verifyElementPropertyValue(response, '[28].title', 'inventore ut quasi magnam itaque est fugit')

WS.verifyElementPropertyValue(response, '[30].userId', '4')
WS.verifyElementPropertyValue(response, '[30].id', '31')
WS.verifyElementPropertyValue(response, '[30].title', 'adipisci laborum fuga laboriosam')

WS.verifyElementPropertyValue(response, '[31].userId', '4')
WS.verifyElementPropertyValue(response, '[31].id', '32')
WS.verifyElementPropertyValue(response, '[31].title', 'reiciendis dolores a ut qui debitis non quo labore')

WS.verifyElementPropertyValue(response, '[52].userId', '6')
WS.verifyElementPropertyValue(response, '[53].id', '54')
WS.verifyElementPropertyValue(response, '[53].title', 'aut non illo amet perferendis')
