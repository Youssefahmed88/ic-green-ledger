import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Input } from "@/components/ui/input"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { AdminLayout } from "@/components/admin/admin-layout"
import { CheckCircle, Clock, Eye, Check, X, Search } from "lucide-react"

const subscriptions = [
  {
    id: "SUB-2024-015",
    farmerName: "James Phiri",
    farmId: "MW-002-2024",
    location: "Blantyre District",
    farmSize: "1.8 hectares",
    policyType: "Flowering Stage Insurance",
    premium: "$9.00",
    submittedDate: "Jan 20, 2024",
    status: "pending",
  },
  {
    id: "SUB-2024-016",
    farmerName: "Grace Mbewe",
    farmId: "MW-003-2024",
    location: "Lilongwe District",
    farmSize: "3.2 hectares",
    policyType: "Full Season Insurance",
    premium: "$41.60",
    submittedDate: "Jan 19, 2024",
    status: "pending",
  },
  {
    id: "SUB-2024-017",
    farmerName: "Peter Kachingwe",
    farmId: "MW-004-2024",
    location: "Mzuzu District",
    farmSize: "2.1 hectares",
    policyType: "Grain Filling Insurance",
    premium: "$9.45",
    submittedDate: "Jan 18, 2024",
    status: "approved",
  },
  {
    id: "SUB-2024-014",
    farmerName: "Mary Tembo",
    farmId: "MW-001-2024",
    location: "Kasungu District",
    farmSize: "2.5 hectares",
    policyType: "Full Season Insurance",
    premium: "$32.50",
    submittedDate: "Jan 17, 2024",
    status: "approved",
  },
  {
    id: "SUB-2024-013",
    farmerName: "John Banda",
    farmId: "MW-005-2024",
    location: "Zomba District",
    farmSize: "4.1 hectares",
    policyType: "Flowering Stage Insurance",
    premium: "$20.50",
    submittedDate: "Jan 16, 2024",
    status: "rejected",
  },
]

export default function SubscriptionsPage() {
  return (
    <AdminLayout>
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Subscription Management</h1>
        <p className="text-gray-600">Review and manage farmer subscription requests</p>
      </div>

      {/* Filters */}
      <Card className="border-0 shadow-lg mb-6">
        <CardContent className="p-6">
          <div className="flex flex-col md:flex-row gap-4">
            <div className="flex-1">
              <div className="relative">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-4 h-4" />
                <Input placeholder="Search by farmer name or farm ID..." className="pl-10" />
              </div>
            </div>
            <Select defaultValue="all">
              <SelectTrigger className="w-48">
                <SelectValue placeholder="Filter by status" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All Status</SelectItem>
                <SelectItem value="pending">Pending</SelectItem>
                <SelectItem value="approved">Approved</SelectItem>
                <SelectItem value="rejected">Rejected</SelectItem>
              </SelectContent>
            </Select>
            <Select defaultValue="all">
              <SelectTrigger className="w-48">
                <SelectValue placeholder="Filter by policy" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All Policies</SelectItem>
                <SelectItem value="full">Full Season Insurance</SelectItem>
                <SelectItem value="flowering">Flowering Stage Insurance</SelectItem>
                <SelectItem value="grain">Grain Filling Insurance</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      {/* Subscriptions List */}
      <Card className="border-0 shadow-lg">
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="text-xl text-gray-900">All Subscriptions</CardTitle>
            <div className="flex space-x-2">
              <Badge variant="secondary" className="bg-orange-100 text-orange-700">
                2 pending
              </Badge>
              <Badge variant="secondary" className="bg-emerald-100 text-emerald-700">
                2 approved
              </Badge>
              <Badge variant="secondary" className="bg-red-100 text-red-700">
                1 rejected
              </Badge>
            </div>
          </div>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            {subscriptions.map((subscription) => (
              <div key={subscription.id} className="border rounded-lg p-6 hover:bg-gray-50">
                <div className="flex items-center justify-between mb-4">
                  <div>
                    <h3 className="text-lg font-semibold text-gray-900">{subscription.farmerName}</h3>
                    <p className="text-sm text-gray-600">
                      {subscription.farmId} • {subscription.location}
                    </p>
                  </div>
                  <Badge
                    variant="outline"
                    className={
                      subscription.status === "pending"
                        ? "text-orange-600 border-orange-200"
                        : subscription.status === "approved"
                          ? "text-emerald-600 border-emerald-200"
                          : "text-red-600 border-red-200"
                    }
                  >
                    {subscription.status === "pending" && <Clock className="w-3 h-3 mr-1" />}
                    {subscription.status === "approved" && <CheckCircle className="w-3 h-3 mr-1" />}
                    {subscription.status === "rejected" && <X className="w-3 h-3 mr-1" />}
                    {subscription.status.charAt(0).toUpperCase() + subscription.status.slice(1)}
                  </Badge>
                </div>

                <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm mb-4">
                  <div>
                    <span className="text-gray-600">Farm Size:</span>
                    <p className="font-medium">{subscription.farmSize}</p>
                  </div>
                  <div>
                    <span className="text-gray-600">Policy Type:</span>
                    <p className="font-medium">{subscription.policyType}</p>
                  </div>
                  <div>
                    <span className="text-gray-600">Premium:</span>
                    <p className="font-medium text-emerald-600">{subscription.premium}</p>
                  </div>
                  <div>
                    <span className="text-gray-600">Submitted:</span>
                    <p className="font-medium">{subscription.submittedDate}</p>
                  </div>
                </div>

                <div className="flex items-center justify-between">
                  <span className="text-xs text-gray-500">ID: {subscription.id}</span>
                  <div className="flex space-x-2">
                    <Button size="sm" variant="outline">
                      <Eye className="w-4 h-4 mr-1" />
                      View Details
                    </Button>
                    {subscription.status === "pending" && (
                      <>
                        <Button size="sm" className="bg-emerald-600 hover:bg-emerald-700">
                          <Check className="w-4 h-4 mr-1" />
                          Approve
                        </Button>
                        <Button
                          size="sm"
                          variant="outline"
                          className="text-red-600 border-red-200 hover:bg-red-50 bg-transparent"
                        >
                          <X className="w-4 h-4 mr-1" />
                          Reject
                        </Button>
                      </>
                    )}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>
    </AdminLayout>
  )
}
